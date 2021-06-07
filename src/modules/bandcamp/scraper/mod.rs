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


#[derive(Debug, Default)]
struct AdditionalProperties {
	duration_secs: Option<f64>,
}


#[derive(Debug, Deserialize)]
struct ApplicationData {
	#[serde(alias = "additionalProperty")]
	#[serde(deserialize_with = "deserialize_properties")]
	additional_properties: AdditionalProperties,
}


fn deserialize_properties<'de, D>(deserializer: D) -> Result<AdditionalProperties, D::Error>
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
			let mut properties = AdditionalProperties::default();

			#[derive(Deserialize)]
			#[serde(untagged)]
			enum Property<'a> {
				FloatProp {
					name: &'a str,
					value: f64,
				},
				None(serde::de::IgnoredAny),
			}

			while let Some(item) = seq.next_element::<Property>()? {
				match item {
					Property::FloatProp { name: "duration_secs", value } => {
						properties.duration_secs = Some(value);
					}

					_ => { }
				}
			}

			Ok(properties)
		}
	}

	deserializer.deserialize_seq(SkipInvalid)
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

	let duration_secs = application_data
		.additional_properties
		.duration_secs
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
