use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::ffi::OsStr;
use std::path::Path;
use std::os::unix::ffi::{OsStrExt, OsStringExt};


pub fn create_unique<P1, P2>(path: &P1, extension: Option<&P2>) -> io::Result<File>
where
	P1: AsRef<Path> + ?Sized,
	P2: AsRef<Path> + ?Sized,
{
	let path = path.as_ref();
	let extension = extension.map(AsRef::as_ref);

	let mut path_buf: Vec<u8> = path
		.as_os_str()
		.to_owned()
		.into_vec();

	let stem_size = path_buf.len();

	let mut try_create = |count: usize| -> io::Result<File> {
		if count > 0 {
			write!(path_buf, " ({})", count)
				.expect("write to vec failed");
		}

		if let Some(extension) = extension {
			path_buf.push(b'.');

			path_buf.extend_from_slice(
				extension
					.as_os_str()
					.as_bytes()
			);
		}

		let filename = OsStr::from_bytes(&path_buf);

		let result = OpenOptions
			::new()
			.write(true)
			.create_new(true)
			.open(filename);

		if result.is_ok() {
			log::debug!("created unique file: {:#?}", Path::new(filename));
		}

		path_buf.truncate(stem_size);

		result
	};

	let mut count: usize = 0;

	loop {
		match try_create(count) {
			Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
				count = count
					.checked_add(1)
					.ok_or(
						io::ErrorKind::AlreadyExists
					)?;
			},

			other => return other,
		};
	}
}
