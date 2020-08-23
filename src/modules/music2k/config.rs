use serde::{Deserialize, Deserializer};

use crate::{
	sim::Sim,
	track::{IdCleaner, Duration},
	util::bytes,
};


#[derive(Debug, Clone)]
pub struct Config {
	pub sim_threshold: Sim,
	pub duration_tolerance: u16,
	pub size_factor: f32,
	pub size_tolerance: f32,
	pub id_cleaner: IdCleaner,
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


impl<'de> Deserialize<'de> for Config {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let config = ConfigFile::deserialize(deserializer)?;

		Ok(
			Config {
				duration_tolerance : config.slizzy.duration_tolerance,
				size_factor        : config.slizzy.size_factor,
				size_tolerance     : config.slizzy.size_tolerance,
				id_cleaner         : config.slizzy.id_clean,
				sim_threshold      : config.music2k.sim_threshold,
			}
		)
	}
}


#[derive(Debug, Deserialize)]
struct ConfigFile {
	slizzy: Slizzy,
	music2k: Music2k,
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
struct Music2k {
	sim_threshold: Sim,
}
