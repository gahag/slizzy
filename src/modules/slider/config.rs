use std::ops::RangeFrom;

use serde::{Deserialize, Deserializer};

use crate::{
	sim::Sim,
	track::Duration
};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
	pub sim_threshold: Sim,
	pub duration_tolerance: u16,
	pub bitrate_range: RangeFrom<u16>,
}


impl Config {
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
}


impl<'de> Deserialize<'de> for Config {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = ConfigFile::deserialize(deserializer)?;

		Ok(
			Config {
				sim_threshold: config.slider.sim_threshold,
				duration_tolerance: config.slizzy.duration_tolerance,
				bitrate_range: RangeFrom { start: config.slizzy.min_bitrate },
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile {
	slizzy: Slizzy,
	slider: Slider,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Slizzy {
	duration_tolerance: u16,
	min_bitrate: u16,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
struct Slider {
	sim_threshold: Sim,
}
