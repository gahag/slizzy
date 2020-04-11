mod str;

use std::convert::{TryInto, TryFrom};

use thiserror::Error;

use derive_more::Into;

use serde::Deserialize;

pub use self::str::str;


#[derive(
	Debug,
	Clone, Copy,
	PartialEq, Eq,
	PartialOrd, Ord,
	Hash,
	Into,
	Deserialize,
)]
pub struct Sim(u8);


impl Sim {
	pub fn value(&self) -> u8 {
		self.0
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ConvertError<T>
where
	T: std::fmt::Debug + std::fmt::Display
{
	#[error("similarity out of range: {0}")]
	OutOfRange(T),
}


impl TryFrom<u8> for Sim {
	type Error = ConvertError<u8>;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		log::trace!("converting Sim: {}", value);

		if value < 100 {
			Ok(
				Sim(value)
			)
		}
		else {
			Err(
				ConvertError::OutOfRange(value)
			)
		}
	}
}


impl TryFrom<i64> for Sim {
	type Error = ConvertError<i64>;

	fn try_from(value: i64) -> Result<Self, Self::Error> {
		log::trace!("converting Sim: {}", value);

		let sim: u8 = value
			.try_into()
			.map_err(|_| ConvertError::OutOfRange(value))?;

		sim
			.try_into()
			.map_err(|_| ConvertError::OutOfRange(value))
	}
}
