mod duration;
mod id;

use std::convert::TryInto;

pub use duration::{Duration, ParseError as ParseDurationError};
pub use id::{Id, Cleaner as IdCleaner, ParseError as ParseIdError};


#[derive(Debug, Clone, Eq)]
pub struct Track {
	pub duration: Option<Duration>,
	id: Id,
	query_string: Box<str>,
}


impl Track {
	pub fn new<I>(id: I) -> Result<Self, ParseIdError>
	where
		I: TryInto<Id>,
		ParseIdError: From<<I as TryInto<Id>>::Error>,
	{
		let id: Id = id.try_into()?;
		let query_string = id.query_string();

		Ok(
			Track {
				id,
				query_string,
				duration: None
			}
		)
	}


	pub fn id(&self) -> &Id { &self.id }


	pub fn query_str(&self) -> &str { &self.query_string }
}


impl PartialEq for Track {
	fn eq(&self, other: &Track) -> bool {
		self.duration == other.duration
			&& self.id == other.id
	}
}


impl std::hash::Hash for Track {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
		self.duration.hash(state);
	}
}


impl std::fmt::Display for Track {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if let Some(len) = self.duration {
			write!(f, "{} [{}]", self.id, len)
		}
		else {
			write!(f, "{}", self.id)
		}
	}
}
