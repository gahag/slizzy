mod config;
mod items;

use std::convert::TryInto;

use lazy_static::lazy_static;

use async_trait::async_trait;

use serde::Deserialize;

use super::websearch;
use crate::net::{
	http,
	url::{self, Url}
};
use items::Items;
pub use config::Config;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
	config: Config,
}


impl super::Module for Module {
	type Config = Config;

	fn new(config: Config) -> Self {
		Module { config }
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct SearchConfig {
	pub custom_search: Box<str>,
}


pub type SearchError = http::Error;


#[async_trait]
impl websearch::Module for Module {
	type SearchConfig = SearchConfig;
	type Error = SearchError;

	async fn search(
		&self,
		query: &str,
		search_config: &Self::SearchConfig
	) -> Result<Box<[Url]>, Self::Error> {
		lazy_static! {
			static ref BASE_URL: url::Dissected<'static> = "https://www.googleapis.com/customsearch/v1"
				.try_into()
				.expect("invalid google api base url");
		}

		let url = BASE_URL
			.clone()
			.extend_query(
				&[
					("q", query),
					("key", &self.config.key),
					("cx", &search_config.custom_search)
				]
			)
			.assemble();

		log::debug!("google query url: {}", url);

		let items = http::Request
			::new(&url)
			.send()
			.await
			.map_err(Into::<http::Error>::into)?
			.body_json::<Items>()
			.await
			.map_err(Into::<http::Error>::into)?;

		let items = items
			.into_iter()
			.filter_map(
				|item| {
					let result = item.parse();

					if result.is_err() {
						log::debug!("google url parse failed: {}", item);
					}

					result.ok()
				}
			)
			.collect();

		Ok(items)
	}
}
