use thiserror::Error;

use super::Id;


pub const SEPARATOR: &'static str = " - ";


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum Error {
	#[error("id too big")]
	TooLong,

	#[error("missing artists")]
	MissingArtists,

	#[error("missing separator")]
	MissingSeparator,

	#[error("missing name")]
	MissingName,

	#[error("missing mix")]
	MissingMix,

	#[error("invalid mix")]
	InvalidMix,

	#[error("trailing characters")]
	TrailingChars,
}


fn parse_artist<'a>(input: &'a str, out: &mut String) -> Result<&'a str, Error> {
	let initial_len = out.len();

	let mut elide_space = true;

	let mut it = input.chars();

	return loop {
		if it.as_str().starts_with(SEPARATOR) { // Lookahead.
			if out.len() == initial_len {
				break Err(Error::MissingArtists);
			}

			if elide_space {
				let space = out.pop();

				assert!(
					space
						.filter(|c| c.is_whitespace())
						.is_some()
				);
			}

			break Ok(it.as_str());
		}

		match it.next() {
			Some(c) if c.is_whitespace() => {
				if !elide_space {
					out.push(c)
				}

				elide_space = true;
			}

			Some('(') | Some(')') => break Err(Error::InvalidMix),

			Some(c) => {
				out.push(c);

				elide_space = false;
			},

			None => break Err(Error::MissingSeparator),
		};

		if out.len() > std::u8::MAX as usize {
			break Err(Error::TooLong);
		}
	};
}


fn parse_name<'a>(input: &'a str, out: &mut String) -> Result<&'a str, Error> {
	let initial_len = out.len();

	let mut elide_space = true;

	let mut it = input.chars();

	let result = loop {
		if it.as_str().starts_with('(') { // Lookahead.
			if out.len() == initial_len {
				break Err(Error::MissingName);
			}

			break Ok(it.as_str());
		}

		match it.next() {
			Some(c) if c.is_whitespace() => {
				if !elide_space {
					out.push(c)
				}

				elide_space = true;
			}

			Some(')') => break Err(Error::InvalidMix),

			Some(c) => {
				out.push(c);

				elide_space = false;
			},

			None => {
				if out.len() == initial_len {
					break Err(Error::MissingName);
				}

				break Ok(it.as_str());
			},
		};

		if out.len() > std::u8::MAX as usize {
			break Err(Error::TooLong);
		}
	}?;

	if elide_space {
		let space = out.pop();

		assert!(
			space
				.filter(|c| c.is_whitespace())
				.is_some()
		);
	}

	Ok(result)
}


fn parse_mix<'a>(input: &'a str, out: &mut String) -> Result<&'a str, Error> {
	let mut elide_space = true;

	let mut open_paren = false;

	let mut it = input.chars();

	return loop {
		match it.next() {
			Some(c) if c.is_whitespace() => {
				if !elide_space {
					out.push(c)
				}

				elide_space = true;
			}

			Some('(') => {
				if open_paren {
					break Err(Error::InvalidMix);
				}

				open_paren = true;

				out.push('(');

				elide_space = true;
			},

			Some(')') => {
				if !open_paren || out.chars().last() == Some('(') {
					break Err(Error::InvalidMix);
				}

				if elide_space {
					let space = out.pop();

					assert!(
						space
							.filter(|c| c.is_whitespace())
							.is_some()
					);
				}

				out.push(')');

				if out.len() > std::u8::MAX as usize {
					break Err(Error::TooLong);
				}

				break Ok(it.as_str());
			},

			Some(c) => {
				if !open_paren {
					break Err(Error::InvalidMix);
				}

				out.push(c);

				elide_space = false;
			},

			None => break Err(Error::MissingMix),
		};

		if out.len() > std::u8::MAX as usize {
			break Err(Error::TooLong);
		}
	};
}


pub fn parse<'a>(input: &'a str) -> Result<(Id, &'a str), Error> {
	let mut id_string = String::new();

	let rest = parse_artist(input, &mut id_string)?;

	let separator = id_string.len() as u8;

	id_string.push_str(SEPARATOR);

	let rest = &rest[SEPARATOR.len()..];

	let rest = parse_name(rest, &mut id_string)?;

	let checkpoint = id_string.len();

	id_string.push(' '); // Prepare to have a mix if available

	let mix = id_string.len() as u8;

	match parse_mix(rest, &mut id_string) {
		Err(Error::MissingMix) => {
			id_string.truncate(checkpoint);

			Ok(
				(
					Id {
						id: id_string.into_boxed_str(),
						separator,
						mix: None
					},
					rest
				)
			)
		},

		result => result.map(
			|rest| (
				Id {
					id: id_string.into_boxed_str(),
					separator,
					mix: Some(mix)
				},
				rest
			)
		),
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	fn check(
		id: &str,
		artists: &str,
		name: &str,
		mix: Option<&str>,
	) -> Result<(), Error> {
		let id: Id = id.parse()?;

		assert_eq!(id.artists(), artists);
		assert_eq!(id.name(), name);
		assert_eq!(id.mix(), mix);

		if let Some(mix) = mix {
			assert_eq!(
				id.title(),
				format!("{} ({})", name, mix)
			);
		}
		else {
			assert_eq!(id.title(), name);
		}

		Ok(())
	}


	#[test]
	fn test_id() -> Result<(), Error> {
		check(
			"Mind Against & Somne - Vertere",
			"Mind Against & Somne",
			"Vertere",
			None
		)?;

		check(
			"Gareth Emery ft. Bo Bruce vs. Bryan Kearney ft. Christina Novelli - U vs. Concrete Angel (Armin van Buuren Mashup)",
			"Gareth Emery ft. Bo Bruce vs. Bryan Kearney ft. Christina Novelli",
			"U vs. Concrete Angel",
			Some("Armin van Buuren Mashup")
		)?;

		check(
			"The Egg ft. Ned Scott & Sophie Barker - Walking Away (Tocadisco Remix, Chemical Disco & Low Cure Bootleg)",
			"The Egg ft. Ned Scott & Sophie Barker",
			"Walking Away",
			Some("Tocadisco Remix, Chemical Disco & Low Cure Bootleg")
		)?;

		check(
			"    Astrix    &    Shpongle   -    Divine   Moments Of Truth (   RollinG & Marcelo Fiorela &    Jotta Soares Remix   )   ",
			"Astrix & Shpongle",
			"Divine Moments Of Truth",
			Some("RollinG & Marcelo Fiorela & Jotta Soares Remix")
		)?;

		Ok(())
	}


	#[test]
	fn test_incorrect_id() {
		assert_eq!(
			"invalid-track".parse::<Id>(),
			Err(Error::MissingSeparator)
		);

		assert_eq!(
			" - invalid track".parse::<Id>(),
			Err(Error::MissingArtists)
		);

		assert_eq!(
			"  - invalid track".parse::<Id>(),
			Err(Error::MissingArtists)
		);

		assert_eq!(
			"invalid - (track)".parse::<Id>(),
			Err(Error::MissingName)
		);

		assert_eq!(
			"invalid -  (track)".parse::<Id>(),
			Err(Error::MissingName)
		);

		assert_eq!(
			"invalid - track(".parse::<Id>(),
			Err(Error::TrailingChars)
		);

		assert_eq!(
			"invalid - track( ".parse::<Id>(),
			Err(Error::TrailingChars)
		);

		assert_eq!(
			"invalid - track(remix".parse::<Id>(),
			Err(Error::TrailingChars)
		);

		assert_eq!(
			"invalid - track  (remix".parse::<Id>(),
			Err(Error::TrailingChars)
		);

		assert_eq!(
			"invalid - track  (remix))".parse::<Id>(),
			Err(Error::TrailingChars)
		);

		assert_eq!(
			"invalid - track()".parse::<Id>(),
			Err(Error::InvalidMix)
		);

		assert_eq!(
			"invalid - track( )".parse::<Id>(),
			Err(Error::InvalidMix)
		);

		assert_eq!(
			"invalid - track(()".parse::<Id>(),
			Err(Error::InvalidMix)
		);

		assert_eq!(
			"invalid - track())".parse::<Id>(),
			Err(Error::InvalidMix)
		);
	}
}
