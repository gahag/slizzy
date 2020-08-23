mod config;
mod source;
pub mod tui;

use async_trait::async_trait;

use crate::track::Track;
use super::tracksource;
pub use config::Config;
pub use source::{
	Error as SourceError,
	Progress as SourceProgress,
	Status,
	ItemStatus,
	Filter
};


#[derive(Debug, Clone)]
pub struct Module {
	config: Config,
}


impl super::Module for Module {
	type Config = Config;

	fn new(config: Config) -> Self {
		Module { config }
	}
}


#[async_trait(?Send)]
impl tracksource::Module for Module {
	type Params = SourceProgress;
	type Error = SourceError;

	async fn fetch(
		&self,
		track: &Track,
		progress: Self::Params
	) -> Result<(), Self::Error> {
		source
			::fetch(self, track, progress)
			.await
	}
}
