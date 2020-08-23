mod scraper;

use std::{
	io,
	path::Path,
};

use thiserror::Error;

use futures::stream::StreamExt;

use crate::{
	report_wrapped,
	track::{Track, Duration},
	net::{http, url::Url},
	sim::{self, Sim},
	web::scraping,
	util,
};
use super::super::{item, websearch};


#[derive(Debug, Error)]
pub enum Error<WSError: std::error::Error> {
	WSError(WSError),
	Items(Box<[ItemError]>),
}


impl<WSError> std::fmt::Display for Error<WSError>
where
	WSError: std::error::Error,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::WSError(error) => write!(f, "websearch error: {}", error),
			Error::Items(errors) => {
				f.write_str("items errors:\n")?;

				for error in errors.iter() {
					writeln!(f, "  {}", error)?
				}

				Ok(())
			}
		}
	}
}


#[derive(Debug, Error)]
pub enum ItemError {
	#[error("http error: {0}")]
	Http(http::Error),

	#[error("scraping error: {0}")]
	Scraping(scraper::Error),

	#[error("download error: {0}")]
	Download(http::downloader::Error),
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
	Id(Sim),
	Blacklist(Box<[Box<str>]>), // array of matched patterns
	Duration(Duration),
	Size(usize),
}


#[derive(Debug)]
pub enum ItemStatus {
	Error(ItemError),
	Expired,
	Filtered(Filter),
	Downloading(util::io::Progress),
	Done
}


#[derive(Debug)]
pub enum Status<WSError: std::error::Error> {
	Fetching,
	NoEntries,
	Error(Error<WSError>),
	Done,
}


pub struct Params<WS: websearch::Module> {
	pub progress: Box<
		dyn item::progress::Progress<
			Id = u8,
			Item = str,
			Status = Status<WS::Error>,
			ItemStatus = ItemStatus,
		> + Send
	>,
	pub websearch: WS,
}


impl<WS> std::fmt::Debug for Params<WS>
where
	WS: websearch::Module + std::fmt::Debug
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Params")
			.field("websearch", &self.websearch)
			.finish()
	}
}


async fn fetch_info(url: &Url) -> Result<scraper::Data, http::Error> {
	let page = http::Request
		::new(url)
		.send()
		.await?
		.body_string()
		.await?;

	let doc = scraping::Html::parse_document(&page);

	Ok(
		scraper::scrap(&doc, url)
	)
}


async fn fetch_preview_duration(url: &Url) -> Result<Duration, ItemError> {
	// 512 bytes should be enough to detect the duration:
	const HEADER_SIZE: usize = 512;
	const RANGE: &'static str = "bytes=0-512";

	let mut mp4_header = Vec::with_capacity(HEADER_SIZE);

	http::Request
		::new(url)
		.append_header(
			http::headers::RANGE,
			http::headers::Value::from_static(RANGE)
		)
		.send()
		.await
		.map_err(
			|error| ItemError::Http(
				http::Error::Request(error)
			)
		)?
		.body_bytes(&mut mp4_header)
		.await
		.map_err(
			|error| ItemError::Http(
				http::Error::Response(error)
			)
		)?;

	let metadata = util::mp4
		::extract_metadata(
			&mut io::Cursor::new(mp4_header)
		)
		.map_err(
			|error| ItemError::Scraping(
				scraping::Error::Format(
					format!("error parsing mp4 header: {}", error).into()
				)
			)
		)?;

	log::debug!("zippy file duration: {}", metadata.duration);

	Ok(metadata.duration)
}


async fn filter_entry<WSConfig>(
	config: &super::Config<WSConfig>,
	track: &Track,
	metadata: scraper::Metadata,
) -> Result<Option<Filter>, ItemError> {
	let id = metadata.id
		.map_err(ItemError::Scraping)?;

	if let Some(id) = id {
		let id = config.id_cleaner.clean(&id);

		let similarity = sim::str(
			&id,
			&track.id()
		);

		if similarity < config.sim_threshold {
			return Ok(
				Some(
					Filter::Id(similarity)
				)
			);
		}

		let blacklist_matches = config.blacklist.matches(&id);

		if blacklist_matches.matched_any() {
			let patterns = util::regex::RegexSetMatches
				::new(&config.blacklist, blacklist_matches)
				.map(Into::into)
				.collect();

			return Ok(
				Some(
					Filter::Blacklist(patterns)
				)
			)
		}
	}

	if let Some(duration) = track.duration {
		let size = metadata.size
			.map_err(ItemError::Scraping)?;

		let size_range = config.size_range(duration);

		if !size_range.contains(&size) {
			return Ok(
				Some(
					Filter::Size(size)
				)
			);
		}

		let preview_url = metadata.preview
			.map_err(ItemError::Scraping)?;

		let preview_duration = fetch_preview_duration(&preview_url).await?;

		let duration_range = config.duration_range(duration);

		if !duration_range.contains(&preview_duration) {
			return Ok(
				Some(
					Filter::Duration(preview_duration)
				)
			);
		}
	}

	Ok(None)
}


async fn handle_item<WSConfig>(
	config: &super::Config<WSConfig>,
	track: &Track,
	url: Url,
	status: impl Fn(&ItemStatus),
) -> Result<(), ItemError> {
	let info = fetch_info(&url)
		.await
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(
					ItemError::Http(error)
				),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	let (download_url, metadata) = match info {
		scraper::Data::Expired => {
			status(&ItemStatus::Expired);
			return Ok(())
		},
		scraper::Data::Available { download, metadata } => (
			download
				.map_err(
					|error| report_wrapped!(
						ItemStatus::Error(
							ItemError::Scraping(error)
						),
						status,
						ItemStatus::Error(error) => error
					)
				)?,
			metadata
		),
	};

	let filter = filter_entry(config, track, metadata)
		.await
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(error),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	if let Some(filter) = filter {
		log::debug!("zippy filtered entry: {:?}", filter);

		status(
			&ItemStatus::Filtered(filter)
		);

		return Ok(())
	}

	let default_path = Path::new(
		track.id().as_ref()
	);

	http::Downloader
		::new()
		.reporter(
			|&progress| status(
				&ItemStatus::Downloading(progress)
			)
		)
		.download_file(&download_url, default_path)
		.await
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(
					ItemError::Download(error)
				),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	status(&ItemStatus::Done);

	Ok(())
}


pub async fn fetch<WS>(
	module: &super::Module<WS>,
	track: &Track,
	params: Params<WS>
) -> Result<(), Error<WS::Error>>
where
	WS: websearch::Module,
{
	let progress = params.progress.as_ref();

	progress.status(&Status::Fetching);

	let urls = params.websearch
		.search(
			track.query_str(),
			&module.config.search
		)
		.await
		.map_err(
			|error| report_wrapped!(
				Status::Error(Error::WSError(error)),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)?;

	log::debug!("zippy urls: {:#?}", urls);

	if urls.is_empty() {
		progress.finish(&Status::NoEntries);
		return Ok(())
	}

	let it = urls
		.into_vec() // box has no owned iterator
		.into_iter();

	progress.size_hint(
		it.size_hint()
	);

	let items: futures::stream::FuturesUnordered<_> = it
		.enumerate()
		.map(
			|(id, url)| {
				let id = id as u8;

				progress.item(id, url.as_ref());

				let progress = &progress;

				handle_item(
					&module.config,
					track,
					url,
					move |status| progress.item_status(id, status)
				)
			}
		)
		.collect();

	let errors: Vec<ItemError> = items
		.filter_map(
			|result| async move {
				result.err()
			}
		)
		.collect()
		.await;

	if errors.is_empty() {
		progress.finish(&Status::Done);
		Ok(())
	}
	else {
		let error = Error::Items(
			errors.into_boxed_slice()
		);

		log::error!("zippy errors: {}", error);

		Err(
			report_wrapped!(
				Status::Error(error),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)
	}
}
