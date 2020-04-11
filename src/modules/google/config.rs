use serde::{Deserialize, Deserializer};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
	pub key: Box<str>,
}


impl<'de> Deserialize<'de> for Config {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = ConfigFile::deserialize(deserializer)?;

		Ok(
			Config {
				key: config.google.key
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile {
	google: Google,
}


#[derive(Debug, Deserialize)]
struct Google {
	key: Box<str>,
}
