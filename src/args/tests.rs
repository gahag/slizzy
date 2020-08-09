use enumset::EnumSet;

use crate::{
	args,
	track::{Duration, Track},
};


fn command_line(line: &str) -> args::Command {
	let arguments = shell_words
		::split(line)
		.expect("failed to shell parse test command line");

	args
		::parse(arguments)
		.expect("failed to parse test command line")
}


#[test]
fn test_debug() {
	let test = |args, level| {
		let commands = [ // (command, track duration)
			(format!("sdl 'Test - track' {}", args), None),
			(format!("sdl {} 'Test - track'", args), None),
			(format!("sdl -d 1:00 'Test - track' {}", args), Some(Duration::from_seconds(60))),
			(format!("sdl -d 1:00 {} 'Test - track'", args), Some(Duration::from_seconds(60))),
			(format!("sdl 'Test - track' -d 1:00 {}", args), Some(Duration::from_seconds(60))),
			(format!("sdl 'Test - track' {} -d 1:00", args), Some(Duration::from_seconds(60))),
		];

		for (command, duration) in commands.iter() {
			let mut track = Track
				::new("Test - track")
				.expect("invalid track");

			track.duration = *duration;

			assert_eq!(
				command_line(command),
				args::Command::Download(
					args::Args {
						log_level: level,
						track,
						metasources: EnumSet::all(),
						tracksources: EnumSet::all(),
					}
				)
			);
		}
	};

	test("", log::Level::Info);
	test("-v", log::Level::Debug);
	test("-vv", log::Level::Trace);
	test("-vvv", log::Level::Trace);
	test("-vvvv", log::Level::Trace);
}
