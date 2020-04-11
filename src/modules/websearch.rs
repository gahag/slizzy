use async_trait::async_trait;

use serde::de::DeserializeOwned;

use crate::net::url::Url;


#[async_trait]
pub trait Module: super::Module {
	type SearchConfig: DeserializeOwned;
	type Error: std::error::Error;

	async fn search(
		&self,
		query: &str,
		config: &Self::SearchConfig
	) -> Result<Box<[Url]>, Self::Error>;
}
