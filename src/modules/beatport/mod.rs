pub mod tui;
mod config;
mod source;
mod scraper;

use async_trait::async_trait;

use crate::track::Track;
use super::{metasource, websearch};
pub use config::Config;
pub use source::{Params as SourceParams, Status, ItemStatus};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
impl<WS> metasource::Module for Module<WS>
where
	WS: websearch::Module,
{
	type Params = SourceParams<WS>;
	type Error = WS::Error;

	async fn fill_metadata(
		&self,
		track: &mut Track,
		params: Self::Params
	) -> Result<bool, Self::Error> {
		source
			::fill_metadata(self, track, params)
			.await
	}
}
