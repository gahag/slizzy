use thiserror::Error;

use serde::Deserialize;

use crate::{
	web::scraping::Html,
	util::bytes,
};


// The bitrate and size fields come from the same element. It's not practical to handle
// them as separate results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct Data {
	pub bitrate: u16,
	pub size: usize
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum Error {
	#[error("missing bitrate")]
	MissingBitrate,

	#[error("invalid bitrate")]
	InvalidBitrate,

	#[error("missing size")]
	MissingSize,

	#[error("invalid size")]
	InvalidSize,
}


pub fn scrap(page: &str) -> Result<Data, Error> {
	let doc = Html::parse_fragment(page);

	log::trace!("scraping html: {:#?}", doc);

	let mut texts = doc
		.root_element()
		.text();

	let bitrate_text = texts
		.nth(1)
		.ok_or(Error::MissingBitrate)?;

	let bitrate: u16 = bitrate_text
		.trim_start()
		.splitn(2, ' ')
		.next()
		.expect("split should always yield at least one item")
		.parse()
		.ok()
		.filter(
			|&bitrate| bitrate > 0
		)
		.ok_or(Error::InvalidBitrate)?;

	let size_text = texts
		.nth(1)
		.ok_or(Error::MissingSize)?;

	let size = size_text
		.parse::<bytes::Mb>()
		.or(Err(Error::InvalidSize))?
		.into();

	Ok(
		Data { bitrate, size }
	)
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_scrap() {
		// <b>Bitrate:</b> 320 kbps <br><b>File Size:</b> 21.16 mb
		assert_eq!(
			scrap("<b>Bitrate:</b> 320 kbps <br><b>File Size:</b> 21.16 mb"),
			Ok(
				Data {
					bitrate: 320,
					size: 22187868
				}
			)
		)
	}
}
