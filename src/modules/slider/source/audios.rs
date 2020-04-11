use std::marker::PhantomData;

use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::track::Duration;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Entry {
	pub id: Box<str>,

	pub url: Box<str>, // This is a url fragment, not a complete url.

	#[serde(rename(deserialize = "tit_art"))]
	pub track_id: Box<str>,

	pub duration: Duration,

	pub extra: Box<str>,
}


// Struct to provide flattened deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Audios(pub Box<[Entry]>);

impl<'de> Deserialize<'de> for Audios {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		Json
			::deserialize(deserializer)
			.map(
				|json| Audios(json.audios.0)
			)
	}
}


// Struct to provide layered deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
struct Json {
	audios: AudiosObject
}


// Struct to flatten audios values.
#[derive(Debug)]
struct AudiosObjectVisitor;

impl<'de> Visitor<'de> for AudiosObjectVisitor {
	type Value = Box<[Entry]>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("audios object")
	}

	fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
		let mut vec: Vec<Entry> = Vec::new();

		let mut next = ||
			map.next_entry_seed::<PhantomData<&str>, EntriesExtender>(
				PhantomData,
				EntriesExtender(&mut vec)
			);

		while let Some(_) = next()? { }

		Ok(
			vec.into_boxed_slice()
		)
	}
}

// Struct to collect audios values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AudiosObject(Box<[Entry]>);

impl<'de> Deserialize<'de> for AudiosObject {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		deserializer
			.deserialize_map(AudiosObjectVisitor)
			.map(AudiosObject)
	}
}


// Struct to push entries into buffer.
#[derive(Debug)]
struct EntriesVisitor<'a>(&'a mut Vec<Entry>);

impl<'de, 'a> Visitor<'de> for EntriesVisitor<'a> {
	type Value = ();

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("audios array")
	}

	fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
		self.0.reserve(
			seq
				.size_hint()
				.unwrap_or(0)
		);

		while let Some(entry) = seq.next_element::<Entry>()? {
			self.0.push(entry);
		}

		Ok(())
	}
}


// Struct to collect entries.
#[derive(Debug)]
struct EntriesExtender<'a>(&'a mut Vec<Entry>);


impl<'de, 'a> DeserializeSeed<'de> for EntriesExtender<'a> {
	// The return type of the `deserialize` method. This implementation
	// appends onto an existing vector but does not create any new data
	// structure, so the return type is ().
	type Value = ();

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_seq(EntriesVisitor(self.0))
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::track::Duration;

	#[test]
	fn test_valid() {
		let payload = std::include_str!("../../../../data/slider/test.json");

		let json = serde_json
			::from_str::<Json>(payload)
			.expect("failed to parse json");

		let expected = Json {
			audios: AudiosObject(
				Box::new(
					[
						Entry {
							id: "371745443_456552853".into(),
							url: "psv4/c815137/u371745443/audios/864ffa53ed2b".into(),
							track_id: "Somne, Mind Against - Vertere".into(),
							duration: Duration::new(9, 14),
							extra: "IUZ-ACRuwtNix4E8bePkI20u2owgeJBf2NFP-ZFwymXCD-fvNnHK3ixzxcWsbhwNWFu0seQBNGG-_aF5-iGy1jdKeRNrsblQ4rZivQmF_sxxUTgUJyarOgprjpTO-wrfMrNS-MWJTp-lS93cUpDFM-0".into(),
						},
						Entry {
							id: "-2001463066_59463066".into(),
							url: "cs1-41v4/p1/844e0fb9227745".into(),
							track_id: "Georgi Z - Vertere".into(),
							duration: Duration::new(6, 24),
							extra: "Iwm_p382q0AB06-n22zGWZ4Oud7cJsPEJt-hvlQa-hrn55sZJ7-VXtB8gxZykPjifMtc6cKADbRPfgmFup05DbZ6g2olAb7wbDGjO8ksaGZCirP3RHeu48eFUOq7yXB_Db-XHaVw_vb3GtVnAw91U8k5".into(),
						},
					],
				)
			)
		};

		assert_eq!(json, expected);
	}
}
