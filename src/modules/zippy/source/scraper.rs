use regex::Regex;

use crate::{
	web::scraping::{Find, FindNext, Html, Text},
	net::url::{Url, PathError},
	util::bytes,
};
pub use crate::web::scraping::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
	// On private files, the track id is an image. It's not worth doing OCR on that.
	pub id: Result<Option<Box<str>>, Error>,
	pub size: Result<usize, Error>,
	// This is a compressed file, so the bitrate is always 64 kbps, but we can use it
	// to discover the duration.
	pub preview: Result<Url, Error>,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Data {
	Expired,
	Available {
		download: Result<Url, Error>,
		metadata: Metadata,
	},
}


pub fn scrap(doc: &Html, url: &Url) -> Data {
	log::trace!("scraping html: {:#?}", doc);

	let expired = doc
		.find_regex("div", "File.* does not exist.* on this server")
		.is_ok();

	if expired {
		Data::Expired
	}
	else {
		Data::Available {
			download: scrap_download(doc, url),
			metadata: Metadata {
				id: scrap_id(doc),
				size: scrap_size(doc),
				preview: scrap_preview(url),
			}
		}
	}
}


fn scrap_size(doc: &Html) -> Result<usize, Error> {
	let text = doc
		.find_regex("font", "Size: ?")?
		.find_next("font")?
		.text_first()?;

	let size = text
		.parse::<bytes::Mb>()
		.or_else(
			|_| Err(
				Error::Format(
					format!("invalid size: '{}'", text).into()
				)
			)
		)?
		.into();

	log::debug!("zippy file size: {}", size);

	Ok(size)
}


fn scrap_id(doc: &Html) -> Result<Option<Box<str>>, Error> {
	let private_file = doc
		.find(r#"img[src ^= "/fileName?key="]"#)
		.is_ok();

	if private_file {
		log::debug!("zippy private file");
		return Ok(None);
	}

	let id = doc
		.find_regex("font", "Name: ?")?
		.find_next("font")?
		.text_first()?
		.into();

	log::debug!("zippy file id: {}", id);

	Ok(Some(id))
}


fn scrap_download(doc: &Html, url: &Url) -> Result<Url, Error> {
	let script = doc
		.find_regex("script", r#"document\.getElementById\('dlbutton'\)\.href *= *"#)?
		.text_first()?;

	log::debug!("zippy download script:\n{}", script);

	let code_regex = Regex
		::new(
			r#"(?x)
				document\.getElementById\('dlbutton'\)\.href \s* = \s*
				"(?P<path1>.*?)"  \s* \+ \s*
				\((?P<expr>.*?)\) \s* \+ \s*
				"(?P<path2>.*?)"
			"#
		)
		.expect("invalid regex");

	let captures = code_regex
		.captures(script)
		.ok_or(
			Error::NotFound(
				"download script".into()
			)
		)?;

	let path1 = &captures["path1"];
	let path2 = &captures["path2"];
	let expr = &captures["expr"];

	let mut expr_namespace = |name: &str, _args: Vec<f64>| -> Option<f64> {
		match name {
			// Custom constants/variables/functions:
			"a" => Some(1.0),
			"b" => Some(2.0),
			"c" => Some(3.0),
			"d" => Some(4.0),

			// A wildcard to handle all undefined names:
			_ => None,
		}
	};

	let expr_result = fasteval
		::ez_eval(expr, &mut expr_namespace)
		.or_else(
			|error| Err(
				Error::Format(
					format!("invalid math expression ({}): {:#?}", expr, error).into()
				)
			)
		)?;

	log::debug!("zippy download script expr result: {}", expr_result);

	let build_url = || -> Result<Url, PathError> {
		let url = url
			.clone()
			.dissect()
			.clear_path()
			.push_path(path1)?
			.push_path(expr_result.to_string())?
			.push_path(
				path2.trim_start_matches('/')
			)?
			.assemble();

		Ok(url)
	};

	let url = build_url()
		.map_err(
			|error| Error::Format(
				format!("invalid url '{}': {}", url, error).into()
			)
		)?;

	log::debug!("zippy download url: {}", url);

	Ok(url)
}


fn scrap_preview(url: &Url) -> Result<Url, Error> {
	let mut dissected_url = url.dissect();

	let key: Box<str> = dissected_url
		.path()
		.nth(1)
		.ok_or(
			Error::Format(
				"missing key from zippy url".into()
			)
		)?
		.into();

	dissected_url
		.push_path("/downloadMusicHQ")
		.map_err(
			|error| Error::Format(
				format!("invalid url '{}': {}", url, error).into()
			)
		)?
		.append_query("key", &key);

	let preview_url = dissected_url.assemble();

	log::debug!("zippy preview url: {}", preview_url);

	Ok(preview_url)
}


// TODO: write tests
