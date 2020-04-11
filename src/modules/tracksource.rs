use async_trait::async_trait;

use enumset::EnumSetType;

use crate::track::Track;


#[async_trait(?Send)]
pub trait Module: super::Module {
	type Params;
	type Error: std::error::Error;

	async fn fetch(
		&self,
		track: &Track,
		params: Self::Params
	) -> Result<(), Self::Error>;
}


#[derive(Debug, Hash, EnumSetType)]
#[enumset(no_ops)]
pub enum TrackSources {
	Music2k,
	Slider,
	Zippy,
}
