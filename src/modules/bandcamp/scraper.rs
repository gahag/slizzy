use crate::{
	track,
	web::scraping::{Find, Html, Text},
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
	title: Result<Box<str>, Error>,
	duration: Result<track::Duration, Error>,
}


pub fn scrap(doc: &Html) -> Data {
	log::trace!("scraping html: {:#?}", doc);

	Data {
		title: scrap_title(doc),
		duration: scrap_duration(doc)
	}
}


fn scrap_duration(doc: &Html) -> Result<track::Duration, Error> {
	let duration = doc
		.find("meta[itemprop = duration]")?
		.value()
		.attr("content")
		.ok_or(
			Error::NotFound(
				"content".into()
			)
		)?;

	let seconds = duration
		.split('.')
		.next()
		.expect("split should always yield at least one item")
		.parse::<u16>()
		.or(
			Err(
				Error::Format(
					"invalid duration".into()
				)
			)
		)?;

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
