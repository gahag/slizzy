use memory_logger::blocking::MemoryLogger;

use regex::Regex;


pub fn setup() -> &'static MemoryLogger {
	MemoryLogger
		::setup(
			log::Level::Debug,
			Regex
				::new("^rslizzy::")
				.expect("invalid logger target regex")
		)
		.expect("log setup should only be called once")
}
