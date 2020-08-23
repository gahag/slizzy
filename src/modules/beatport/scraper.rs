use crate::{
	track,
	web::scraping::{Attr, Find, Html, Text},
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
	pub track_id: Result<Box<str>, Error>,
	pub duration: Result<track::Duration, Error>,
}


pub fn scrap(doc: &Html) -> Data {
	log::trace!("scraping html: {:#?}", doc);

	Data {
		track_id: scrap_id(doc),
		duration: scrap_duration(doc)
	}
}


fn scrap_duration(doc: &Html) -> Result<track::Duration, Error> {
	doc
		.find("li.interior-track-length > span.value")?
		.text_first()?
		.parse()
		.or(
			Err(
				Error::Format(
					"invalid track duration".into()
				)
			)
		)
}


fn scrap_id(doc: &Html) -> Result<Box<str>, Error> {
	let artists = doc
		.find("div.interior-track-actions")?
		.attr("data-ec-d1")?;

	let name = doc
		.find("div.interior-title > h1")?
		.text_first()?;

	let mix = doc
		.find("div.interior-title > h1.remixed")
		.and_then(
			|e| e.text_first()
		)
		.ok()
		.filter(|&s| s != "Original Mix");

	Ok(
		if let Some(mix) = mix {
			format!("{} - {} ({})", artists, name, mix)
				.into_boxed_str()
		}
		else {
			format!("{} - {}", artists, name)
				.into_boxed_str()
		}
	)
}
