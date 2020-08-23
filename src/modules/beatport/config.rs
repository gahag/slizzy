use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use crate::{
	sim::Sim,
	track::IdCleaner,
};


#[derive(Debug, Clone)]
pub struct Config<SearchConfig> {
	pub sim_threshold: Sim,
	pub search: SearchConfig,
	pub id_cleaner: IdCleaner,
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
				id_cleaner: config.slizzy.id_clean,
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile<SearchConfig> where SearchConfig: DeserializeOwned {
	slizzy: Slizzy,

	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	beatport: Beatport<SearchConfig>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Slizzy {
	id_clean: IdCleaner,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Beatport<SearchConfig> where SearchConfig: DeserializeOwned {
	sim_threshold: Sim,

	#[serde(flatten)]
	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	search: SearchConfig,
}
