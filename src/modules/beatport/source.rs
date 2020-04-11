use thiserror::Error;

use futures::stream::StreamExt;

use crate::{
	report_wrapped,
	net::{url::Url, http},
	sim::{self, Sim},
	track::{Track, Duration},
	web::scraping,
};
use super::scraper;
use super::super::{item, websearch};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status<WSError: std::error::Error> {
	Skipped,
	Searching,
	Done,
	MatchNotFound,
	Error(WSError),
}


#[derive(Debug, Error)]
pub enum ItemError {
	#[error("http error: {0}")]
	Http(http::Error),

	#[error("title not found: {0}")]
	TitleNotFound(scraping::Error),

	#[error("duration not found: {0}")]
	DurationNotFound(scraping::Error),
}


#[derive(Debug)]
pub enum ItemStatus {
	Selected,
	TitleMismatch(Sim),
	Error(ItemError),
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


async fn scrap(url: &Url) -> Result<scraper::Data, http::Error> {
	let page = http::Request
		::new(url)
		.send()
		.await?
		.body_string()
		.await?;

	let doc = scraping::Html::parse_document(&page);

	Ok(
		scraper::scrap(&doc)
	)
}


fn select<WSError>(
	id: u8,
	item: Result<scraper::Data, http::Error>,
	track_title: &str,
	sim_threshold: Sim,
	progress: &dyn item::progress::Progress<
		Id = u8,
		Item = str,
		Status = Status<WSError>,
		ItemStatus = ItemStatus,
	>
) -> Option<Duration>
where
	WSError: std::error::Error,
{
	let report_error = |error| progress.item_status(
		id,
		&ItemStatus::Error(error)
	);

	let item = item
		.map_err(
			|error| report_error(ItemError::Http(error))
		)
		.ok()?;

	let title = item.title
		.map_err(
			|error| report_error(ItemError::TitleNotFound(error))
		)
		.ok()?;

	let duration = item.duration
		.map_err(
			|error| report_error(ItemError::DurationNotFound(error))
		)
		.ok()?;

	progress.item(
		id,
		&format!("{} [{}]", title, duration)
	);

	let similarity = sim::str(&title, track_title);

	if similarity >= sim_threshold {
		progress.item_status(id, &ItemStatus::Selected);

		Some(duration)
	}
	else {
		progress.item_status(
			id,
			&ItemStatus::TitleMismatch(similarity)
		);

		None
	}
}


pub async fn fill_metadata<WS>(
	module: &super::Module<WS>,
	track: &mut Track,
	params: Params<WS>
) -> Result<bool, WS::Error>
where
	WS: websearch::Module,
{
	let title = track
		.id()
		.title();

	let progress = params.progress.as_ref();

	if track.duration.is_none() {
		progress.status(&Status::Searching);
	}
	else {
		progress.finish(&Status::Skipped);
		return Ok(true);
	}

	let urls = params.websearch
		.search(
			track.query_str(),
			&module.config.search
		)
		.await
		.map_err(
			|error| report_wrapped!(
				Status::Error(error),
				|status| progress.finish(status),
				Status::Error(error) => error
			)
		)?;

	log::debug!("beatport urls: {:#?}", urls);

	let urls = urls
		.into_vec() // box has no owned iterator
		.into_iter();

	progress.size_hint(
		urls.size_hint()
	);

	let mut pages: futures::stream::FuturesUnordered<_> = urls
		.enumerate()
		.map(
			|(id, url)| {
				let id = id as u8;

				progress.item(id, url.as_ref());

				async move {
					(
						id,
						scrap(&url).await
					)
				}
			}
		)
		.collect();

	while let Some((id, item)) = pages.next().await {
		let duration = select(id, item, title, module.config.sim_threshold, progress);

		if duration.is_some() {
			track.duration = duration;

			progress.finish(&Status::Done);

			return Ok(true)
		}
	}

	progress.finish(&Status::MatchNotFound);

	Ok(false)
}
