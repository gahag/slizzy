use std::ffi::OsString;

use clap::{clap_app, crate_authors, crate_version, crate_description};

use enumset::EnumSet;

use crate::{
	modules::{metasource::MetaSources, tracksource::TrackSources},
	track::Track,
};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
	Help(Box<str>),
	Version(Box<str>),
	Download(Args)
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Args {
	pub track: Track,
	pub metasources: EnumSet<MetaSources>,
	pub tracksources: EnumSet<TrackSources>,
}


pub fn parse<A, T>(args: A) -> clap::Result<Command>
where
	A: IntoIterator<Item = T>,
	T: Into<OsString> + Clone
{
	let app = clap_app!(
		slizzy =>
			(version: crate_version!())
			(author: crate_authors!())
			(about: crate_description!())
			(@arg id: +required "the track id to download")
			(@arg duration: -d --duration +takes_value "specify the track duration")
			// metasources:
			(@arg beatport: --beatport "Use the beatport module")
			(@arg bandcamp: --bandcamp "Use the bandcamp module")
			// tracksources:
			(@arg slider:  --slider  "Use the slider module")
			(@arg music2k: --music2k "Use the music2k module")
			(@arg zippy:   --zippy   "Use the zippy module")
	);

	match app.get_matches_from_safe(args) {
		Ok(matches) => {
			let track = parse_track(&matches)?;

			Ok(
				Command::Download(
					Args {
						track,
						metasources: parse_metasources(&matches),
						tracksources: parse_tracksources(&matches),
					}
				)
			)
		},

		Err(error) => match error.kind {
			clap::ErrorKind::HelpDisplayed => Ok(
				Command::Help(error.message.into_boxed_str())
			),
			clap::ErrorKind::VersionDisplayed => Ok(
				Command::Version(error.message.into_boxed_str())
			),
			_ => Err(error)
		}
	}
}


fn parse_track(matches: &clap::ArgMatches) -> clap::Result<Track> {
	let mut track = Track
		::new(
			matches
				.value_of("id")
				.expect("id parameter is required")
		)
		.map_err(
			|error| clap::Error::with_description(
				&format!("invalid track id: {}", error),
				clap::ErrorKind::ValueValidation
			)
		)?;

	if let Some(duration) = matches.value_of("duration") {
		let duration = duration
			.parse()
			.map_err(
				|error: crate::track::ParseDurationError| clap::Error::with_description(
					&format!("invalid track duration: {}", error),
					clap::ErrorKind::ValueValidation
				)
			)?;

		track.duration = Some(duration);
	}

	Ok(track)
}


fn parse_metasources(matches: &clap::ArgMatches) -> EnumSet<MetaSources> {
	let mut sources = EnumSet::new();

	if matches.is_present("beatport") {
		sources.insert(MetaSources::Beatport);
	}

	if matches.is_present("bandcamp") {
		sources.insert(MetaSources::Bandcamp);
	}

	if sources.is_empty() {
		EnumSet::all()
	}
	else {
		sources
	}
}


fn parse_tracksources(matches: &clap::ArgMatches) -> EnumSet<TrackSources> {
	let mut sources = EnumSet::new();

	if matches.is_present("slider") {
		sources.insert(TrackSources::Slider);
	}

	if matches.is_present("music2k") {
		sources.insert(TrackSources::Music2k);
	}

	if matches.is_present("zippy") {
		sources.insert(TrackSources::Zippy);
	}

	if sources.is_empty() {
		EnumSet::all()
	}
	else {
		sources
	}
}
