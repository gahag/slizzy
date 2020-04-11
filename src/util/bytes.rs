use std::str::FromStr;

use lazy_static::lazy_static;

use derive_more::Display;

use regex::Regex;


pub const MB: usize = 1 * 1024 * 1024;


#[derive(Debug, Display, Clone, Copy, PartialEq, PartialOrd)]
#[display(fmt = "{:.2} MB", _0)]
pub struct Mb(pub f32);


impl From<usize> for Mb {
	fn from(size: usize) -> Self {
		Mb(
			(size as f32) / (MB as f32)
		)
	}
}


impl Into<usize> for Mb {
	fn into(self) -> usize {
		(self.0 * (MB as f32)) as usize
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseError;


impl FromStr for Mb {
	type Err = ParseError;

	fn from_str(text: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref PATTERN: Regex = Regex
				::new(r#"^([0-9]+(?:\.[0-9]*)?) *(?i:mb)"#)
				.expect("invalid regex");
		}

		let number = PATTERN
			.captures(
				text.trim_start()
			)
			.ok_or(ParseError)?
			.get(1)
			.ok_or(ParseError)?
			.as_str();

		let size: f32 = number
			.parse()
			.or(Err(ParseError))?;

		Ok(
			Mb(size)
		)
	}
}


impl std::convert::TryFrom<&str> for Mb {
	type Error = ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() {
		assert_eq!(
			"10.1 Mb".parse(),
			Ok(Mb(10.1))
		);
		assert_eq!(
			"10.1 MB".parse(),
			Ok(Mb(10.1))
		);
		assert_eq!(
			"10.1 mb".parse(),
			Ok(Mb(10.1))
		);
		assert_eq!(
			"10.1Mb".parse(),
			Ok(Mb(10.1))
		);
		assert_eq!(
			"0 Mb".parse(),
			Ok(Mb(0.0))
		);
	}
}
