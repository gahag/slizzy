use memory_logger::blocking::MemoryLogger;

use regex::Regex;


pub fn setup(level: log::Level) -> &'static MemoryLogger {
	MemoryLogger
		::setup(
			level,
			Regex
				::new("^rslizzy::")
				.expect("invalid logger target regex")
		)
		.expect("log setup should only be called once")
}
