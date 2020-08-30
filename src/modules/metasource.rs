use async_trait::async_trait;

use enumset::EnumSetType;

use crate::track::Track;


#[async_trait(?Send)]
pub trait Module: super::Module {
	type Params;
	type Error: std::error::Error;

	async fn fill_metadata(
		&self,
		track: &mut Track,
		params: Self::Params
	) -> Result<bool, Self::Error>;
}


#[derive(Debug, Hash, EnumSetType)]
#[enumset(no_ops)]
pub enum MetaSources {
	Bandcamp,
	Beatport,
}
