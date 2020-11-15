#[cfg(test)]
mod tests;

use serde::Deserialize;

use crate::{
	track,
	web::scraping::{Find, Html, Text},
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
	pub title: Result<Box<str>, Error>,
	pub duration: Result<track::Duration, Error>,
}


pub fn scrap(doc: &Html) -> Data {
	log::trace!("scraping html: {:#?}", doc);

	Data {
		title: scrap_title(doc),
		duration: scrap_duration(doc)
	}
}


#[derive(Debug, Deserialize)]
struct ApplicationData {
	duration_secs: f64,
}


fn scrap_duration(doc: &Html) -> Result<track::Duration, Error> {
	let json = doc
		.find("script[type = 'application/ld+json']")?
		.text_first()?;

	let application_data: ApplicationData = serde_json
		::from_str(json)
		.map_err(
			|error| Error::Format(
				format!("failed to parse json: {}", error).into()
			)
		)?;

	let duration_secs = application_data.duration_secs;

	if !(u16::MIN as f64 ..= u16::MAX as f64).contains(&duration_secs) {
		return Err(
			Error::Format(
				format!("duration out of range: {}", duration_secs).into()
			)
		)
	}

	let seconds = application_data.duration_secs as u16;

	Ok(
		track::Duration::new(0, seconds)
	)
}


fn scrap_title(doc: &Html) -> Result<Box<str>, Error> {
	let title = doc
		.find("h2.trackTitle")?
		.text_first()?
		.into();

	Ok(title)
}
