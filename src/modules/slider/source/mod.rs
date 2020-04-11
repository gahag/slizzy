mod audios;
mod scraper;

use std::{
	convert::TryInto,
	path::Path
};

use thiserror::Error;

use derive_more::Deref;

use lazy_static::lazy_static;

use futures::stream::StreamExt;

use crate::{
	track::{Track, Duration},
	net::{
		http,
		url::{self, Url}
	},
	sim::{self, Sim},
	util,
	report_wrapped,
};
use super::super::item;
use audios::{Audios, Entry};
pub use self::scraper::Data;


lazy_static! {
	static ref BASE_URL: url::Dissected<'static> = "https://slider.kz"
		.try_into()
		.expect("invalid url");
}


#[derive(Debug, Error)]
pub enum Error {
	Http(http::Error),
	Items(Box<[ItemError]>),
}


impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::Http(error) => write!(f, "http error: {}", error),
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filter {
	Id(Sim),
	Duration(Duration),
	Bitrate(u16),
}


#[derive(Debug)]
pub enum ItemStatus {
	Attempt {
		number: usize,
		error: ItemError,
	},
	Error(ItemError),
	Filtered(Filter),
	Downloading(util::io::Progress),
	Done
}


#[derive(Debug)]
pub enum Status {
	Fetching,
	Attempt {
		number: usize,
		error: http::Error,
	},
	Error(Error),
	Done,
}


#[derive(Deref)]
// TODO replace with type synonym when https://github.com/rust-lang/rust/issues/63033
// gets solved.
pub struct Progress(
	pub Box<
		dyn item::progress::Progress<
			Id = u8,
			Item = str,
			ItemStatus = ItemStatus,
			Status = Status,
		> + Send
	>
);


async fn fetch_info(entry: &Entry) -> Result<Data, ItemError> {
	let url = BASE_URL
		.clone()
		.extend_path(
			&[
				"info",
				&entry.id,
				&entry.duration.as_seconds().to_string(),
				"",
			]
		)
		.expect("invalid url")
		.push_path(
			format!("{}.mp3", entry.url)
		)
		.expect("invalid url")
		.append_query("extra", &entry.extra)
		.assemble();

	log::debug!("info url: {}", url);

	let page = http::Request
		::new(&url)
		.send()
		.await
		.map_err(
			|error| ItemError::Http(
				error.into()
			)
		)?
		.body_string()
		.await
		.map_err(
			|error| ItemError::Http(
				error.into()
			)
		)?;

	log::trace!("info page: {}", page);

	scraper::scrap(&page)
		.map_err(ItemError::Scraping)
}


async fn fetch_entries(query_string: &str) -> Result<Box<[Entry]>, http::Error> {
	let url = BASE_URL
		.clone()
		.push_path("vk_auth.php")
		.expect("invalid url")
		.append_query("q", query_string)
		.assemble();

	log::debug!("slider url: {}", url);

	let entries = http::Request
		::new(&url)
		.send()
		.await?
		.body_json::<Audios>()
		.await?
		.0;

	log::trace!("slider entries: {:#?}", entries);

	Ok(entries)
}


async fn filter_entry(
	config: &super::Config,
	track: &Track,
	entry: &Entry,
	status: &impl Fn(&ItemStatus),
) -> Result<Option<Filter>, ItemError> {
	if let Some(duration) = track.duration {
		let duration_range = config.duration_range(duration);

		if !duration_range.contains(&entry.duration) {
			return Ok(
				Some(
					Filter::Duration(entry.duration)
				)
			);
		}
	}

	let similarity = sim::str(
		&entry.track_id,
		&track.id()
	);

	if similarity < config.sim_threshold {
		return Ok(
			Some(
				Filter::Id(similarity)
			)
		);
	}

	let info = util::future
		::retry(
			|| fetch_info(&entry),

			|error| matches!(error, ItemError::Scraping(_)),

			|number, error| {
				report_wrapped!(
					ItemStatus::Attempt { number, error },
					status,
					ItemStatus::Attempt { error, .. } => error
				)
			},

			Some(10)
		)
		.await
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(error),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	if !config.bitrate_range.contains(&info.bitrate) {
		return Ok(
			Some(
				Filter::Bitrate(info.bitrate)
			)
		);
	}

	Ok(None)
}


async fn handle_item(
	config: &super::Config,
	track: &Track,
	entry: Entry,
	status: impl Fn(&ItemStatus),
) -> Result<(), ItemError> {
	let filter = filter_entry(config, track, &entry, &status).await?;

	if let Some(filter) = filter {
		status(
			&ItemStatus::Filtered(filter)
		);

		return Ok(())
	}

	let build_url = || -> Result<Url, url::PathError> {
		Ok(
			BASE_URL
				.clone()
				.extend_path(
					&[
						"download",
						&entry.id,
						&entry.duration.as_seconds().to_string(),
					]
				)?
				.push_path(&entry.url)?
				.push_path(
					format!("{}.mp3", entry.track_id)
				)?
				.append_query("extra", &entry.extra)
				.assemble()
		)
	};

	let url = build_url()
		.expect("invalid url");

	log::debug!("download url: {}", url);

	let default_path = Path::new(
		entry.track_id.as_ref()
	);

	http::Downloader
		::new()
		.reporter(
			|&progress| status(
				&ItemStatus::Downloading(progress)
			)
		)
		.download_file(&url, default_path)
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


pub async fn fetch(
	module: &super::Module,
	track: &Track,
	progress: Progress
) -> Result<(), Error> {
	progress.status(&Status::Fetching);

	let query_str = track.query_str();

	let entries = util::future
		::retry(
			|| fetch_entries(&query_str),

			|error| matches!(error, http::Error::Request(_)),

			|number, error| report_wrapped!(
				Status::Attempt { number, error },
				|status| progress.status(status),
				Status::Attempt { error, .. } => error
			),

			None
		)
		.await
		.map_err(
			|error| report_wrapped!(
				Status::Error(
					Error::Http(error)
				),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)?;

	log::debug!("slider entries: {:#?}", entries);

	let it = entries
		.into_vec() // box has no owned iterator
		.into_iter();

	progress.size_hint(
		it.size_hint()
	);

	let items: futures::stream::FuturesUnordered<_> = it
		.enumerate()
		.map(
			|(id, entry)| {
				let id = id as u8;

				progress.item(id, &entry.track_id);

				let progress = &progress;

				handle_item(
					&module.config,
					track,
					entry,
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

		log::error!("slider errors: {}", error);

		Err(
			report_wrapped!(
				Status::Error(error),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)
	}
}
