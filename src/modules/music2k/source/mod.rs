mod scraper;

use std::{
	convert::TryInto,
	path::Path,
	ops::Deref,
};

use thiserror::Error;

use derive_more::Deref;

use lazy_static::lazy_static;

use futures::stream::StreamExt;

use crate::{
	report_wrapped,
	track::{Track, Duration},
	net::{http, url::{self, Url}},
	sim::{self, Sim},
	web::scraping,
	util,
};
use super::super::item;


lazy_static! {
	static ref BASE_URL: url::Dissected<'static> = "https://music2k.com/"
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
	Id(Sim),
	Duration(Duration),
	Size(usize),
}


#[derive(Debug)]
pub enum ItemStatus {
	Error(ItemError),
	Filtered(Filter),
	Downloading(util::io::Progress),
	Done
}


#[derive(Debug)]
pub enum Status {
	Fetching,
	NoEntries,
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
			Status = Status,
			ItemStatus = ItemStatus,
		> + Send
	>
);


async fn fetch_size(url: &Url) -> Result<usize, ItemError> {
	let response = http::Request
		::new(url)
		.set_method(http::request::Method::HEAD)
		.send()
		.await
		.map_err(
			|error| ItemError::Http(
				http::Error::Request(error)
			)
		)?;

	let size = response
		.content_length()
		.ok_or(
			ItemError::Scraping(
				scraping::Error::Format(
					"CONTENT_LENGTH header missing from response".into()
				)
			)
		)?;

	Ok(size)
}


async fn fetch_entries(query_string: &str) -> Result<scraper::Entries, http::Error> {
	let url = BASE_URL
		.clone()
		.extend_path(&[
			"s",
			query_string,
		])
		.expect("invalid url")
		.assemble();

	log::debug!("music2k url: {}", url);

	let mut response = http::Request
		::new(&url)
		.set_require_success(false)
		.send()
		.await?;

	match response.status() {
		status if status.as_u16() == 404 =>  Ok(
			scraper::Entries(Default::default())
		),

		status if status.is_success() => {
			let page = response
				.body_string()
				.await?;

			let doc = scraping::Html::parse_document(&page);

			Ok(
				scraper::scrap(&doc)
			)
		},

		status => Err(
			http::Error::Request(
				http::request::Error::status(&status)
			)
		)
	}
}


async fn filter_entry(
	config: &super::Config,
	track: &Track,
	entry: scraper::Entry,
) -> Result<Option<Filter>, ItemError> {
	let id = entry.id
		.map_err(ItemError::Scraping)?;

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

	if let Some(duration) = track.duration {
		let entry_duration = entry.duration
			.map_err(ItemError::Scraping)?;

		let duration_range = config.duration_range(duration);

		if !duration_range.contains(&entry_duration) {
			return Ok(
				Some(
					Filter::Duration(entry_duration)
				)
			);
		}

		let download_url = entry.download
			.map_err(ItemError::Scraping)?;

		let size = fetch_size(&download_url).await?;

		let size_range = config.size_range(duration);

		if !size_range.contains(&size) {
			return Ok(
				Some(
					Filter::Size(size)
				)
			);
		}
	}

	Ok(None)
}


async fn handle_item(
	config: &super::Config,
	track: &Track,
	entry: scraper::Entry,
	status: impl Fn(&ItemStatus),
) -> Result<(), ItemError> {
	let download_url = entry.download
		.clone()
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(
					ItemError::Scraping(error)
				),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	let name = entry.id
		.as_ref()
		.map(Deref::deref)
		.unwrap_or(
			track
				.id()
				.as_ref()
		)
		.to_owned();

	let filter = filter_entry(config, track, entry)
		.await
		.map_err(
			|error| report_wrapped!(
				ItemStatus::Error(error),
				status,
				ItemStatus::Error(error) => error
			)
		)?;

	if let Some(filter) = filter {
		log::debug!("music2k filtered entry: {:?}", filter);

		status(
			&ItemStatus::Filtered(filter)
		);

		return Ok(())
	}

	let default_path = Path::new(&name);

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


pub async fn fetch(
	module: &super::Module,
	track: &Track,
	progress: Progress
) -> Result<(), Error> {
	progress.status(&Status::Fetching);

	let entries =
		fetch_entries(
			track.query_str()
		)
		.await
		.map_err(
			|error| report_wrapped!(
				Status::Error(Error::Http(error)),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)?;

	log::debug!("music2k entries: {:#?}", entries);

	if entries.is_empty() {
		progress.finish(&Status::NoEntries);
		return Ok(())
	}

	let it = entries.0
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

				progress.item(
					id,
					entry.id
						.as_ref()
						.map(Deref::deref)
						.unwrap_or("missing title")
				);

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

		log::error!("music2k errors: {}", error);

		Err(
			report_wrapped!(
				Status::Error(error),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)
	}
}
