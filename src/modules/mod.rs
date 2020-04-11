pub mod websearch;
pub mod metasource;
pub mod tracksource;
pub mod item;
pub mod google;
pub mod beatport;
pub mod bandcamp;
pub mod slider;
pub mod zippy;
pub mod music2k;


use serde::de::DeserializeOwned;


pub trait Module: Sized {
	type Config: DeserializeOwned;

	fn new(cfg: Self::Config) -> Self;
}
