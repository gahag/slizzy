use std::{
	io::{self, Read, Write, ErrorKind},
	fs::File,
	path::Path
};

use thiserror::Error;

use serde::de::DeserializeOwned;


const TEMPLATE: &'static str = std::include_str!("config.toml");


#[derive(Debug, Error)]
pub enum LoadError {
	#[error("xdg error: {0}")]
	Xdg(xdg::BaseDirectoriesError),

	#[error("io error: {0}")]
	Io(io::Error),
}


impl LoadError {
	pub fn is_not_found(&self) -> bool {
		if let LoadError::Io(error) = self {
			error.kind() == ErrorKind::NotFound
		}
		else {
			false
		}
	}
}


pub fn load_from(path: &Path) -> Result<Box<str>, io::Error>
{
	let mut content = String::with_capacity(512);

	File
		::open(path)?
		.read_to_string(&mut content)?;

	Ok(content.into_boxed_str())
}


pub fn load() -> Result<Box<str>, LoadError>
{
	let xdg_dirs = xdg::BaseDirectories
		::with_prefix("slizzy")
		.map_err(LoadError::Xdg)?;

	let path = xdg_dirs
		.place_config_file("config.toml")
		.map_err(LoadError::Io)?;

	if path.exists() {
		load_from(&path)
			.map_err(LoadError::Io)
	}
	else {
		File
			::create(path)
			.map_err(LoadError::Io)?
			.write_all(
				TEMPLATE.as_bytes()
			)
			.map_err(LoadError::Io)?;

		Err(
			LoadError::Io(
				io::Error::new(ErrorKind::NotFound, "config file not found, template placed")
			)
		)
	}
}


#[derive(Debug, Error)]
#[error("invalid config file: {0}")]
pub struct ReadError(toml::de::Error);


pub fn read<Config, C>(config: C) -> Result<Config, ReadError>
where
	C: AsRef<str>,
	Config: DeserializeOwned,
{
	let config = config.as_ref();

	log::trace!("deserializing config from data: {}", config);

	toml
		::from_str(config)
		.map_err(ReadError)
}
