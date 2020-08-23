mod cleaner;
mod parser;

use lazy_static::lazy_static;

use derive_more::{Deref, Display};

use regex::Regex;

pub use cleaner::Cleaner;
pub use parser::Error as ParseError;


//         id
// /-----------------\
//             title
//          /--------\
// artists - name (mix)
#[derive(Debug, Display, Clone, Eq, Ord, Deref)]
#[deref(forward)]
#[display(fmt = "{}", id)]
pub struct Id {
	id: Box<str>,

	#[deref(ignore)]
	separator: u8,

	#[deref(ignore)]
	mix: Option<u8>, // '(' index
}


impl Id {
	pub fn artists(&self) -> &str {
		&self.id[.. (self.separator as usize)]
	}


	pub fn name(&self) -> &str {
		if let Some(mix) = self.mix {
			&self.id[self.after_separator() .. (mix - 1) as usize]
		}
		else {
			&self.id[self.after_separator()..]
		}
	}


	pub fn mix(&self) -> Option<&str> {
		let mix = self.mix? as usize;

		Some(
			&self.id[(mix + 1) .. (self.id.len() - 1)]
		)
	}


	pub fn title(&self) -> &str {
		&self.id[self.after_separator()..]
	}


	pub fn query_string(&self) -> Box<str> {
		lazy_static! {
			static ref NOISE_PATTERN: Regex = Regex
				::new(r"( +(ft|vs)\.? +)|( *(&|-|\(|\)|,) *)")
				.expect("invalid regex");
		}

		let mut result = NOISE_PATTERN
			.replace_all(&self.id, " ")
			.into_owned();

		result.truncate( // The mix's closing parenthesis gets replaced by a space.
			result
				.trim_end()
				.len()
		);

		log::debug!("query string for {}: {}", self, result);

		result.into_boxed_str()
	}


	fn after_separator(&self) -> usize {
		self.separator as usize + parser::SEPARATOR.len()
	}
}


impl PartialEq for Id {
	fn eq(&self, other: &Id) -> bool {
		self.id == other.id
	}
}


impl PartialOrd for Id {
	fn partial_cmp(&self, other: &Id) -> Option<std::cmp::Ordering> {
		self.id.partial_cmp(&other.id)
	}
}


impl std::hash::Hash for Id {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}


impl AsRef<str> for Id {
	fn as_ref(&self) -> &str { &self.id }
}


impl std::borrow::Borrow<str> for Id {
	fn borrow(&self) -> &str { &self.id }
}


impl std::str::FromStr for Id {
	type Err = ParseError;

	fn from_str<'a>(id: &'a str) -> Result<Self, Self::Err> {
		log::debug!("parsing track id: {}", id);

		let (id, rest) = parser::parse(id)?;

		if rest.trim_start().is_empty() {
			Ok(id)
		}
		else {
			Err(ParseError::TrailingChars)
		}
	}
}


impl std::convert::TryFrom<&str> for Id {
	type Error = ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}
