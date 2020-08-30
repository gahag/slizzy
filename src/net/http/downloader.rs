use std::io;
use std::path::Path;
use std::fs::File;

use thiserror::Error;

use futures::AsyncReadExt;

use crate::{
	net::{http, url::Url},
	util,
};


#[derive(Debug, Clone, Copy)]
pub struct Downloader<R> where R: FnMut(&util::io::Progress) {
	buffer_size: usize,
	reporter: Option<R>,
}


#[derive(Debug, Error)]
pub enum Error {
	#[error("http error: {0}")]
	Http(http::Error),

	#[error("io error: {0}")]
	Io(io::Error),
}


impl<R> Downloader<R> where R: FnMut(&util::io::Progress) {
	pub fn new() -> Self {
		Self {
			buffer_size: 8 * 1024,
			reporter: None,
		}
	}


	pub fn reporter(mut self, reporter: R) -> Self {
		self.reporter = Some(reporter);
		self
	}


	pub fn buffer(mut self, size: usize) -> Self {
		log::debug!("buffer size: {}", size);

		self.buffer_size = size;
		self
	}


	fn report(&mut self, progress: &util::io::Progress) {
		self.reporter
			.as_mut()
			.map(
				|reporter| reporter(&progress)
			);
	}


	fn get_target_file(response: &http::Response, default: &Path) -> io::Result<File> {
		log::debug!("default filename: {:#?}", default);

		let filename = response.filename();
		let filename = filename.as_deref();

		log::debug!("response filename: {:#?}", filename);

		let filename = filename
			.and_then(Path::file_name)
			.map(Path::new)
			.unwrap_or(default);

		let stem = filename
			.file_stem()
			.expect("missing filename from default path");

		let extension = filename
			.extension()
			.filter(
				|ext| !ext.is_empty()
			)
			.or_else(
				|| {
					let mime = response.mime()?;

					util::mime
						::guess_extension(mime)
						.map(AsRef::as_ref)
				}
			);

		util::io::fs::file::create_unique(stem, extension)
	}


	async fn download_body<W>(
		&mut self,
		response: &mut http::Response,
		mut out: W
	) -> Result<(), Error>
	where
		W: io::Write
	{
		let mut progress = util::io::Progress {
			total: response.content_length(),
			completed: 0,
		};

		let mut buffer: Box<[u8]> = vec![0; self.buffer_size].into_boxed_slice();

		loop {
			let bytes_read = response
				.read(&mut buffer)
				.await
				.map_err(Error::Io)?;

			if bytes_read == 0 {
				return match progress.total {
					Some(total) if progress.completed != total => Err(
						Error::Http(
							http::Error::Response(
								anyhow::anyhow!("download finished abruptly").into()
							)
						)
					),
					Some(_) => Ok(()),
					None => Ok(()),
				}
			}

			out
				.write_all(&buffer[..bytes_read])
				.map_err(Error::Io)?;

			progress.completed += bytes_read;

			self.report(&progress);
		}
	}


	pub async fn download<W>(&mut self, url: &Url, out: W) -> Result<(), Error>
	where
		W: io::Write
	{
		// Report 0 before dispatching the request:
		self.report(
			&util::io::Progress::default()
		);

		let mut response = http::Request
			::new(url)
			.send()
			.await
			.map_err(
				|error| Error::Http(
					error.into()
				)
			)?;

		log::trace!("download response: {:#?}", response);

		self
			.download_body(&mut response, out)
			.await
	}


	pub async fn download_file(
		&mut self,
		url: &Url,
		default_path: &Path
	) -> Result<(), Error> {
		let log_failed = || log::warn!("download failed: {}", url);

		// Report 0 before dispatching the request:
		self.report(
			&util::io::Progress::default()
		);

		let mut response = http::Request
			::new(url)
			.send()
			.await
			.map_err(
				|error| {
					log_failed();

					Error::Http(
						error.into()
					)
				}
			)?;

		log::trace!("download response: {:#?}", response);

		let file = Downloader::<R>
			::get_target_file(&response, default_path)
			.map_err(
				|error| {
					log_failed();
					Error::Io(error)
				}
			)?;

		log::trace!("download file: {:#?}", file);

		let result = self
			.download_body(&mut response, file)
			.await;

		if result.is_err() {
			log_failed()
		}

		result
	}
}
