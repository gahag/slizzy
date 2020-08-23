use lazy_static::lazy_static;

use derive_more::Deref;

use crate::{
	track::Duration,
	web::scraping::{Attr, ElementRef, Find, Html, Selector, Text},
	net::url::Url,
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
	pub id: Result<Box<str>, Error>,
	pub duration: Result<Duration, Error>,
	pub download: Result<Url, Error>,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref)]
pub struct Entries(pub Box<[Entry]>);


pub fn scrap(doc: &Html) -> Entries {
	log::trace!("scraping html: {:#?}", doc);

	lazy_static! {
		static ref TABLE_SELECTOR: Selector = Selector
			::parse(
				"table.songs tr"
			)
			.expect("Invalid css selector");
	}

	let rows = doc.select(&TABLE_SELECTOR);

	Entries(
		rows
			.map(scrap_entry)
			.collect()
	)
}


fn scrap_entry(entry: ElementRef) -> Entry {
	Entry {
		id: scrap_id(entry),
		duration: scrap_duration(entry),
		download: scrap_download(entry),
	}
}


fn scrap_id(entry: ElementRef) -> Result<Box<str>, Error> {
	lazy_static! {
		static ref NAME_SELECTOR: Selector = Selector
			::parse(
				"td.name a.item"
			)
			.expect("Invalid css selector");
	}

	let mut items = entry.select(&NAME_SELECTOR);

	let mut next_item = || {
		items
			.next()
			.ok_or(
				Error::NotFound(
					"entry name".into()
				)
			)
	};

	let artists =
		next_item()?
		.text_first()?;

	let title =
		next_item()?
		.text_first()?;

	let id = format!("{} - {}", artists, title);

	log::debug!("music2k entry id: {}", id);

	Ok(
		id.into_boxed_str()
	)
}


fn scrap_duration(entry: ElementRef) -> Result<Duration, Error> {
	let text = entry
		.find("td.time")?
		.text_first()?;

	let duration = text
		.parse::<Duration>()
		.or_else(
			|_| Err(
				Error::Format(
					format!("invalid duration: '{}'", text).into()
				)
			)
		)?
		.into();

	log::debug!("music2k entry duration: {}", duration);

	Ok(duration)
}


fn scrap_download(entry: ElementRef) -> Result<Url, Error> {
	let url = entry
		.find("td.download > a.i-dl")?
		.attr("href")?;

	let url = url
		.parse::<Url>()
		.map_err(
			|error| Error::Format(
				format!("invalid url '{}': {}", url, error).into()
			)
		)?;

	log::debug!("music2k entry url: {}", url);

	Ok(url)
}


// TODO: write tests
