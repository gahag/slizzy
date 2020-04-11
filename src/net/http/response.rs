use std::{
	io,
	borrow::Cow,
	os::unix::ffi::{OsStrExt, OsStringExt},
	path::Path,
	pin::Pin,
	task::{Context, Poll},
	ffi::{OsStr, OsString},
};

use lazy_static::lazy_static;

use derive_more::From;

use thiserror::Error;

use serde::de::DeserializeOwned;

use regex::bytes::Regex;

use futures::AsyncReadExt;

pub use isahc::http::{
	StatusCode,
	header::{HeaderMap, HeaderName, HeaderValue}
};

use super::headers;


#[derive(Debug)]
pub struct Response(
	pub(super) isahc::http::Response<isahc::Body>
);


#[derive(Debug, From, Error)]
#[error("{0}")]
pub struct Error(anyhow::Error);


impl Response {
	pub fn status(&self) -> StatusCode {
		self.0.status()
	}


	pub fn headers(&self) -> &HeaderMap {
		self.0.headers()
	}


	pub fn content_length(&self) -> Option<usize> {
		let content_length = self
			.headers()
			.get(headers::CONTENT_LENGTH)?;

		log::debug!("content length: {:#?}", content_length);

		let total_size = content_length
			.to_str()
			.ok()?
			.parse()
			.ok()?;

		Some(total_size)
	}


	pub fn filename(&self) -> Option<Cow<Path>> {
		lazy_static! {
			static ref FILENAME_REGEX: Regex = Regex
				::new(r#"attachment; filename\*?=(UTF-8''|")?(?P<filename>[^"]*)"?"#)
				.expect("invalid filename regex");
		}

		let content_disposition = self
			.headers()
			.get(headers::CONTENT_DISPOSITION)?;

		log::debug!("Content-Disposition header: {:#?}", content_disposition);

		let filename = FILENAME_REGEX
			.captures(
				content_disposition.as_bytes()
			)?
			.name("filename")?
			.as_bytes();

		if filename.is_empty() {
			None
		}
		else {
			let filename_decoded = match percent_encoding::percent_decode(filename).into() {
				Cow::Borrowed(filename_decoded) => Cow::Borrowed(
					Path::new(
						OsStr::from_bytes(filename_decoded)
					)
				),
				Cow::Owned(filename_decoded) => Cow::Owned(
					OsString::from_vec(filename_decoded).into()
				),
			};

			Some(filename_decoded)
		}
	}


	pub fn mime(&self) -> Option<&str> {
		let content_type = self
			.headers()
			.get(headers::CONTENT_TYPE)?;

		log::debug!("Content-Type header: {:#?}", content_type);

		let mime = content_type
			.as_bytes()
			.splitn(
				2,
				|&c| c == b';'
			)
			.next()
			.expect("splitn should yield at least once");

		std::str
			::from_utf8(mime)
			.ok()
	}


	pub async fn body_bytes(&mut self, body: &mut Vec<u8>) -> Result<usize, Error> {
		self.0
			.body_mut()
			.read_to_end(body)
			.await
			.map_err(
				|error| Error(
					error.into()
				)
			)
	}


	pub async fn body_json<T: DeserializeOwned>(&mut self) -> Result<T, Error> {
		let mut body = Vec::with_capacity(1024);

		self.body_bytes(&mut body)
			.await
			.map_err(Into::<anyhow::Error>::into)?;

		log::trace!(
			"response json payload: {}",
			String::from_utf8_lossy(&body)
		);

		serde_json
			::from_slice(&body)
			.map_err(Into::into)
			.map_err(Error)
	}


	pub async fn body_string(&mut self) -> Result<Box<str>, Error> {
		let mut string = String::with_capacity(8);

		self.0
			.body_mut()
			.read_to_string(&mut string)
			.await
			.map_err(Into::<anyhow::Error>::into)?;

		Ok(
			string.into_boxed_str()
		)
	}
}


impl futures::AsyncRead for Response {
	fn poll_read(
		mut self: Pin<&mut Self>,
		cx: &mut Context,
		buf: &mut [u8]
	) -> Poll<Result<usize, io::Error>> {
		let body = self.0.body_mut();

		Pin
			::new(body)
			.poll_read(cx, buf)
	}
}
