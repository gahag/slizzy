use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use regex::RegexSet;

use crate::{
	sim::Sim,
	track::{IdCleaner, Duration},
	util::bytes,
};


#[derive(Debug, Clone)]
pub struct Config<SearchConfig> {
	pub search: SearchConfig,
	pub sim_threshold: Sim,
	pub duration_tolerance: u16,
	pub size_factor: f32,
	pub size_tolerance: f32,
	pub id_cleaner: IdCleaner,
	pub blacklist: RegexSet,
}


impl<SearchConfig> Config<SearchConfig> {
	pub fn duration_range(&self, duration: Duration) -> std::ops::Range<Duration> {
		let seconds = duration.as_seconds();

		std::ops::Range {
			start: Duration::from_seconds(
				seconds.saturating_sub(self.duration_tolerance)
			),
			end: Duration::from_seconds(
				seconds.saturating_add(self.duration_tolerance)
			),
		}
	}


	pub fn size_range(&self, duration: Duration) -> std::ops::Range<usize> {
		let seconds = duration.as_seconds();

		let minutes = seconds as f32 / 60.0;

		let expected_size = minutes * self.size_factor;

		std::ops::Range {
			start: bytes::Mb(expected_size - self.size_tolerance).into(),
			end:   bytes::Mb(expected_size + self.size_tolerance).into(),
		}
	}
}


impl<'de, SearchConfig> Deserialize<'de> for Config<SearchConfig>
where
	SearchConfig: DeserializeOwned,
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = ConfigFile::deserialize(deserializer)?;

		let blacklist = RegexSet
			::new(
				config.zippyshare.blacklist.into_vec() // Box has no owned iterator
			)
			.map_err(serde::de::Error::custom)?;

		Ok(
			Config {
				duration_tolerance : config.slizzy.duration_tolerance,
				size_factor        : config.slizzy.size_factor,
				size_tolerance     : config.slizzy.size_tolerance,
				id_cleaner         : config.slizzy.id_clean,
				search             : config.zippyshare.search,
				sim_threshold      : config.zippyshare.sim_threshold,
				blacklist,
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile<SearchConfig> where SearchConfig: DeserializeOwned {
	slizzy: Slizzy,
	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	zippyshare: Zippy<SearchConfig>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Slizzy {
	duration_tolerance: u16,
	size_factor: f32,
	size_tolerance: f32,
	id_clean: IdCleaner,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Zippy<SearchConfig> where SearchConfig: DeserializeOwned {
	sim_threshold: Sim,

	#[serde(flatten)]
	#[serde(bound(deserialize = "SearchConfig: DeserializeOwned"))]
	search: SearchConfig,

	blacklist: Box<[Box<str>]>,
}
