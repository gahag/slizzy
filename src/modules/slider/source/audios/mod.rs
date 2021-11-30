#[cfg(test)]
mod tests;

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

	pub extra: Option<Box<str>>,
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
