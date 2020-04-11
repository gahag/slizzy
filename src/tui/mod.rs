pub mod main;

use std::io::{self, Read};

use termion::{
	event::Event,
	raw::{RawTerminal, IntoRawMode},
	screen::AlternateScreen
};
pub use termion::event::Key;

use tui::backend::TermionBackend;
pub use tui::Terminal;


pub type Backend = TermionBackend<
	AlternateScreen<
		RawTerminal<io::Stdout>
	>
>;


pub fn terminal() -> io::Result<Terminal<Backend>> {
	Terminal::new(
		TermionBackend::new(
			AlternateScreen::from(
				io
					::stdout()
					.into_raw_mode()?
			)
		)
	)
}


pub struct StdinReader(termion::AsyncReader);


impl StdinReader {
	pub fn new() -> Self {
		Self(
			termion::async_stdin()
		)
	}


	pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
		let mut buffer: [u8; 1] = [ 0 ];

		let bytes_read = self.0.read(&mut buffer)?;

		if bytes_read == 0 {
			Ok(None)
		}
		else {
			Ok(
				Some(buffer[0])
			)
		}
	}


	pub fn read_key(&mut self) -> io::Result<Option<Key>> {
		struct Iter<'a>(&'a mut StdinReader);

		impl<'a> Iterator for Iter<'a> {
			type Item = io::Result<u8>;

			fn next(&mut self) -> Option<Self::Item> {
				self.0
					.read_byte()
					.transpose()
			}
		}

		let byte = match self.read_byte()? {
			Some(byte) => byte,
			None => return Ok(None),
		};

		let event = termion::event::parse_event(byte, &mut Iter(self))?;

		match event {
			Event::Key(key) => Ok(Some(key)),
			_ => Ok(None),
		}
	}


	pub fn read_keys<'a>(&'a mut self) -> impl Iterator<Item = io::Result<Key>> + 'a
	{
		struct Iter<'a>(&'a mut StdinReader);

		impl<'a> Iterator for Iter<'a>
		{
			type Item = io::Result<Key>;

			fn next(&mut self) -> Option<Self::Item> {
				self.0
					.read_key()
					.transpose()
			}
		}

		Iter(self)
	}


	pub fn read_action<'a, A>(&'a mut self) -> impl Iterator<Item = io::Result<A>> + 'a
	where
		Key: Into<A>,
	{
		self
			.read_keys()
			.map(
				|result| result.map(Into::into)
			)
	}
}


impl std::fmt::Debug for StdinReader {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StdinReader")
			.finish()
	}
}
