use crate::{
	track,
	web::scraping::{Find, Html, Text},
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data { // TODO scrap artists
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


fn scrap_title(doc: &Html) -> Result<Box<str>, Error> {
	let name = doc
		.find("div.interior-title > h1")?
		.text_first()?
		.into();

	let mix = doc
		.find("div.interior-title > h1.remixed")
		.and_then(
			|e| e.text_first()
		)
		.ok()
		.filter(|&s| s != "Original Mix");

	Ok(
		if let Some(mix) = mix {
			format!("{} ({})", name, mix).into_boxed_str()
		}
		else {
			name
		}
	)
}
