use std::convert::TryFrom;

use derive_more::From;

use thiserror::Error;

pub use isahc::{
	config::{Configurable, RedirectPolicy},
	http::{
		Method,
		header::{HeaderMap, HeaderName, HeaderValue},
		StatusCode,
	},
};

use super::{headers, Response};
use super::super::url::Url;


#[derive(Debug, From, Error)]
#[error("{0}")]
pub struct Error(anyhow::Error);


impl Error {
	pub fn status(status: &StatusCode) -> Self {
		Self(
			anyhow::anyhow!("status is not success: {:#?}", status)
		)
	}
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'a> {
	method: Method,
	url: &'a Url,
	headers: HeaderMap,
	body: Vec<u8>,
	require_success: bool,
}


impl<'a> Request<'a> {
	pub fn new(url: &'a Url) -> Self {
		let mut headers = HeaderMap::with_capacity(1);

		headers.append(
			headers::USER_AGENT,
			HeaderValue::from_static(
				"Mozilla/5.0 (Windows NT 10.0; WOW64; rv:77.0) Gecko/20100101 Firefox/77.0"
			)
		);

		Self {
			method: Method::GET,
			url,
			headers,
			body: Default::default(),
			require_success: true
		}
	}


	pub fn method(&self) -> &Method {
		&self.method
	}


	pub fn set_method(mut self, method: Method) -> Self {
		self.method = method;
		self
	}


	pub fn url(&self) -> &Url {
		self.url
	}


	pub fn set_url(mut self, url: &'a Url) -> Self {
		self.url = url;
		self
	}


	pub fn headers(&self) -> &HeaderMap {
		&self.headers
	}


	pub fn headers_mut(&mut self) -> &mut HeaderMap {
		&mut self.headers
	}


	pub fn append_header(mut self, name: HeaderName, value: HeaderValue) -> Self
	{
		self.headers.append(name, value);
		self
	}


	pub fn extend_headers<H>(mut self, headers: H) -> Self
	where
		H: IntoIterator<Item = (HeaderName, HeaderValue)>
	{
		self.headers.extend(headers);
		self
	}


	pub fn body(&self) -> &[u8] {
		self.body.as_ref()
	}


	pub fn body_mut(&mut self) -> &mut Vec<u8> {
		&mut self.body
	}


	pub fn with_body(mut self, body: Vec<u8>) -> Self {
		self.body = body;
		self
	}


	pub fn set_require_success(mut self, value: bool) -> Self {
		self.require_success = value;
		self
	}


	pub async fn send(self) -> Result<Response, Error> {
		log::debug!("dispatching request to {}", self.url);

		// We shouldn't have to clone. Poor design on the http crate.
		let uri = isahc::http::Uri
			::try_from(
				self.url.as_ref()
			)
			.expect("invaldi url");

		let (mut parts, body) = isahc::http::Request
			::builder()
			.redirect_policy(RedirectPolicy::Limit(5))
			.ssl_options(isahc::config::SslOption::DANGER_ACCEPT_INVALID_CERTS)
			.method(self.method)
			.uri(uri)
			.body(self.body)
			.expect("invalid request")
			.into_parts();

		parts.headers = self.headers;

		let request = isahc::http::Request::from_parts(parts, body);

		let response = isahc
			::send_async(request)
			.await
			.map(Response)
			.map_err(Into::<anyhow::Error>::into)?;

		if self.require_success && !response.0.status().is_success() {
			return Err(
				Error(
					anyhow::anyhow!("status is not success: {:#?}", response.0)
				)
			)
		}

		Ok(response)
	}
}
