use std::{
	convert::TryInto,
	io::{self, Read, Seek, SeekFrom}
};

use crate::track::Duration;


// MP4 reference:
// https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFChap1/qtff1.html


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Metadata {
	pub duration: Duration,
}


pub fn extract_metadata<R>(header: R) -> io::Result<Metadata>
where
	R: Read + Seek
{
	const VERSION_SIZE           : u32 = 1;
	const FLAGS_SIZE             : u32 = 3;
	const CREATION_TIME_SIZE     : u32 = 4;
	const MODIFICATION_TIME_SIZE : u32 = 4;
	const TIME_SCALE_SIZE        : u32 = 4;
	const DURATION_SIZE          : u32 = 4;

	let mut reader = HeaderReader::new(header);

	reader.find_atom(*b"moov")?;
	reader.find_atom(*b"mvhd")?;

	reader.skip(VERSION_SIZE + FLAGS_SIZE + CREATION_TIME_SIZE + MODIFICATION_TIME_SIZE)?;

	let timescale = reader.read_u32()?;
	let duration = reader.read_u32()?;

	Ok(
		Metadata {
			duration: Duration::from_seconds(
				(duration / timescale) as u16
			),
		}
	)
}


#[derive(Debug)]
struct HeaderReader<R> {
	reader: R,
	buffer: Vec<u8>,
}


impl<R> HeaderReader<R> where R: Read + Seek {
	pub fn new(reader: R) -> Self {
		Self {
			reader,
			buffer: Vec::with_capacity(64),
		}
	}


	pub fn clear(&mut self) {
		self.buffer.clear();
	}


	pub fn buffer(&self) -> &[u8] {
		self.buffer.as_ref()
	}


	pub fn read_exact(&mut self, size: usize) -> io::Result<()> {
		self.buffer.resize_with(size, Default::default);

		self.reader.read_exact(&mut self.buffer[0..size])
	}


	pub fn read_u32(&mut self) -> io::Result<u32> {
		self.read_exact(4)?;

		Ok(
			u32::from_be_bytes(
				(&self.buffer[0..4])
					.try_into()
					.expect("failed to cast slice")
			)
		)
	}


	pub fn read_size(&mut self) -> io::Result<u32> {
		let size = self.read_u32()?;

		Ok(size - 4) // disconsider the number of bytes read
	}


	pub fn read_atom_type(&mut self) -> io::Result<&[u8]> {
		self.read_exact(4)?;

		Ok(
			self.buffer()
		)
	}


	pub fn find_atom(&mut self, id: [u8; 4]) -> io::Result<u32> {
		let mut atom_size = self.read_size()?;

		while self.read_atom_type()? != id {
			self.skip(
				atom_size - 4 // Disconsider the number of bytes read.
			)?;
			atom_size = self.read_size()?;
		}

		Ok(atom_size - 4) // Disconsider the number of bytes read.
	}


	pub fn skip(&mut self, len: u32) -> io::Result<()> {
		self.reader
			.seek(
				SeekFrom::Current(len as i64)
			)?;

		Ok(())
	}
}
