use thiserror::Error;

use serde::Deserialize;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Duration(u16);

impl Duration {
	pub fn new(minutes: u16, seconds: u16) -> Self {
		Self(minutes * 60 + seconds)
	}


	pub fn from_seconds(seconds: u16) -> Self {
		Self(seconds)
	}


	pub fn split(&self) -> (u16, u16) {
		(
			self.0 / 60,
			self.0 % 60
		)
	}


	pub fn as_seconds(&self) -> u16 {
		self.0
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ParseError {
	#[error("missing minutes")]
	MissingMinutes,

	#[error("missing seconds")]
	MissingSeconds,

	#[error("invalid number")]
	InvalidNumber,

	#[error("invalid seconds")]
	InvalidSeconds
}


impl From<std::time::Duration> for Duration {
	fn from(duration: std::time::Duration) -> Self {
		Self::from_seconds(
			duration.as_secs() as u16
		)
	}
}


impl std::str::FromStr for Duration {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		log::debug!("parsing Duration: {}", s);

		let mut parts = s.splitn(2, ':');

		let mins = parts
			.next()
			.ok_or(ParseError::MissingMinutes)?;

		let secs = parts
			.next()
			.ok_or(ParseError::MissingSeconds)?;

		assert!( // splitn by 2 must yield at most 2 elements.
			parts
				.next()
				.is_none()
		);

		let mins = mins
			.parse::<u16>()
			.or(
				Err(ParseError::InvalidNumber)
			)?;

		if secs.len() != 2 {
			return Err(ParseError::InvalidSeconds);
		}

		let secs = secs
			.parse::<u16>()
			.or(
				Err(ParseError::InvalidNumber)
			)?;

		if secs >= 60 {
			return Err(ParseError::InvalidSeconds);
		}

		Ok(
			Duration::new(mins, secs)
		)
	}
}


impl std::convert::TryFrom<&str> for Duration {
	type Error = ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}


impl std::fmt::Debug for Duration {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let (mins, secs) = self.split();

		write!(f, "Duration {}:{:02}", mins, secs)
	}
}


impl std::fmt::Display for Duration {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let (mins, secs) = self.split();

		write!(f, "{}:{:02}", mins, secs)
	}
}
