mod config;
mod source;
pub mod tui;

use async_trait::async_trait;

use crate::track::Track;
use super::{tracksource, websearch};
pub use config::Config;
pub use source::{
	Error as SourceError,
	Params as SourceParams,
	Status,
	ItemStatus,
	Filter
};


#[derive(Debug, Clone)]
pub struct Module<WS: websearch::Module> {
	config: Config<WS::SearchConfig>,
}


impl<WS: websearch::Module> super::Module for Module<WS> {
	type Config = Config<WS::SearchConfig>;

	fn new(config: Self::Config) -> Self {
		Module { config }
	}
}


#[async_trait(?Send)]
impl<WS> tracksource::Module for Module<WS>
where
	WS: websearch::Module,
{
	type Params = SourceParams<WS>;
	type Error = SourceError<WS::Error>;

	async fn fetch(
		&self,
		track: &Track,
		params: Self::Params
	) -> Result<(), Self::Error> {
		source
			::fetch(self, track, params)
			.await
	}
}
