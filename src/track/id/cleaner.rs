use std::borrow::Cow;

use serde::{Deserialize, Deserializer};

use regex::Regex;


#[derive(Debug, Clone)]
pub struct Cleaner {
	pattern: Regex,
}


impl Cleaner {
	pub fn clean<'a>(&self, id: &'a str) -> Cow<'a, str> {
		self.pattern.replace_all(id, "")
	}
}


impl<'de> Deserialize<'de> for Cleaner
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = Box::<[Box<str>]>::deserialize(deserializer)?;

		let mut pattern = String::with_capacity(200);

		pattern.push_str("(?i)"); // Case insensitive.

		for pat in config.iter() {
			pattern.push('(');
			pattern.push_str(pat);
			pattern.push_str(")|");
		}

		pattern.pop();

		let pattern = Regex
			::new(&pattern)
			.map_err(serde::de::Error::custom)?;

		Ok(
			Cleaner {
				pattern,
			}
		)
	}
}
