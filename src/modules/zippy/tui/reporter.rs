use std::{
	error::Error,
	marker::PhantomData,
	sync,
};

use super::{progress, BackendStatus, BackendItemStatus};
use super::status::{Status, ItemStatus};


pub type Message = progress::Message<u8, Box<str>, Status, ItemStatus>;


#[derive(Debug)]
pub struct Reporter<WSError: Error + 'static> {
	tx: sync::mpsc::Sender<Message>,
	wserror: PhantomData<&'static WSError>,
}


impl<WSError: Error> Reporter<WSError> {
	pub fn new(tx: sync::mpsc::Sender<Message>) -> Self {
		Self {
			tx,
			wserror: PhantomData,
		}
	}
}


impl<WSError: Error> progress::Progress for Reporter<WSError> {
	type Id = u8;
	type Item = str;
	type Status = BackendStatus<WSError>;
	type ItemStatus = BackendItemStatus;


	fn size_hint(&self, hint: (usize, Option<usize>)) {
		self.tx
			.send(
				Message::SizeHint(hint.0, hint.1)
			)
			.expect("channel closed before backend finished");
	}


	fn item(&self, id: Self::Id, item: &Self::Item) {
		self.tx
			.send(
				Message::Item(
					id,
					item.into()
				)
			)
			.expect("channel closed before backend finished");
	}


	fn item_status(&self, id: Self::Id, status: &Self::ItemStatus) {
		self.tx
			.send(
				Message::ItemStatus(
					id,
					status.into()
				)
			)
			.expect("channel closed before backend finished");
	}


	fn status(&self, status: &Self::Status) {
		self.tx
			.send(
				Message::Status(
					status.into()
				)
			)
			.expect("channel closed before backend finished");
	}


	fn finish(&self, status: &Self::Status) {
		self.tx
			.send(
				Message::Finish(
					status.into()
				)
			)
			.expect("channel closed before backend finished");
	}
}
