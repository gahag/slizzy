pub fn guess_extension(mime: &str) -> Option<&'static str> {
	let extensions = mime_guess::get_mime_extensions_str(mime)?;

	log::debug!("extensions for {}: {:#?}", mime, extensions);

	let extension = match *extensions.first()? {
		"m2a" => "mp3",
		other => other,
	};

	log::debug!("selected extension: {}", extension);

	Some(extension)
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_guess_extension() {
		let _ = simple_logger::init(); // Enable loggin to stderr for this test.

		assert_eq!(
			guess_extension("audio/mpeg"),
			Some("mp3")
		);

		assert_eq!(
			guess_extension("audio/wav"),
			Some("wav")
		);

		assert_eq!(
			guess_extension("audio/aac"),
			Some("aac")
		);

		assert_eq!(
			guess_extension("application/x-download"),
			None
		);
	}
}
