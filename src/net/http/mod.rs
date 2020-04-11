pub mod request;
pub mod response;
pub mod downloader;

use derive_more::From;

use thiserror::Error;

pub use request::Request;
pub use response::Response;
pub use downloader::Downloader;


pub mod headers {
	pub use isahc::http::header::{
		HeaderValue as Value,
		CONTENT_DISPOSITION,
		CONTENT_LENGTH,
		CONTENT_TYPE,
		RANGE,
		USER_AGENT
	};
}


#[derive(Debug, From, Error)]
pub enum Error {
	#[error("response error: {0}")]
	Request(request::Error),

	#[error("request error: {0}")]
	Response(response::Error),
}
