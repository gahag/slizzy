#[cfg(test)]
mod tests;

use std::fmt;

use serde::Deserialize;

use crate::{
	track,
	web::scraping::{Find, Html, Text},
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
	/// This can be the track id or title, depending on the label.
	pub track: Result<Box<str>, Error>,
	pub duration: Result<track::Duration, Error>,
}


pub fn scrap(doc: &Html) -> Data {
	log::trace!("scraping html: {:#?}", doc);

	Data {
		track: scrap_track(doc),
		duration: scrap_duration(doc)
	}
}


#[derive(Debug, Deserialize)]
struct AdditionalProperty {
	name: String,
	value: f64,
}


#[derive(Debug)]
struct AdditionalProperties(Box<[AdditionalProperty]>);

impl<'de> serde::Deserialize<'de> for AdditionalProperties {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>
	{
		struct SkipInvalid;

		impl<'de> serde::de::Visitor<'de> for SkipInvalid {
			type Value = AdditionalProperties;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("additionalProperty array")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				#[derive(Deserialize)]
				#[serde(untagged)]
				enum NoneOnError {
					Some(AdditionalProperty),
					None(serde::de::IgnoredAny),
				}

				let mut vec = Vec::new();

				while let Some(item) = seq.next_element::<NoneOnError>()? {
					if let NoneOnError::Some(value) = item {
						vec.push(value);
					}
				}

				Ok(
					AdditionalProperties(vec.into())
				)
			}
		}

		deserializer.deserialize_seq(SkipInvalid)
	}
}


#[derive(Debug, Deserialize)]
struct ApplicationData {
	#[serde(alias = "additionalProperty")]
	additional_properties: AdditionalProperties,
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

	let duration_secs = application_data.additional_properties.0
		.iter()
    .find(|prop| prop.name == "duration_secs")
    .map(|prop| prop.value)
    .ok_or_else(
			|| Error::Format("missing duration_secs property".into())
		)?;

	if !(u16::MIN as f64 ..= u16::MAX as f64).contains(&duration_secs) {
		return Err(
			Error::Format(
				format!("duration out of range: {}", duration_secs).into()
			)
		)
	}

	let seconds = duration_secs as u16;

	Ok(
		track::Duration::new(0, seconds)
	)
}


fn scrap_track(doc: &Html) -> Result<Box<str>, Error> {
	let track = doc
		.find("h2.trackTitle")?
		.text_first()?
		.into();

	Ok(track)
}
