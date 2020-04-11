use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use crate::sim::Sim;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config<SearchConfig> {
	pub sim_threshold: Sim,
	pub search: SearchConfig,
}


impl<'de, SearchConfig> Deserialize<'de> for Config<SearchConfig>
where
	SearchConfig: DeserializeOwned,
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = ConfigFile::deserialize(deserializer)?;

		Ok(
			Config {
				sim_threshold: config.beatport.sim_threshold,
				search: config.beatport.search,
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile<SearchConfig> where SearchConfig: DeserializeOwned {
	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	beatport: Beatport<SearchConfig>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Beatport<SearchConfig> where SearchConfig: DeserializeOwned {
	sim_threshold: Sim,

	#[serde(flatten)]
	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	search: SearchConfig,
}
