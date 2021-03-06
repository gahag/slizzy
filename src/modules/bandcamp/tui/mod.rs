mod renderer;
mod reporter;
mod status;

use std::error::Error;
use std::sync;

use super::{Status as BackendStatus, ItemStatus as BackendItemStatus};
use super::super::item::progress;
use status::{Status, ItemStatus};
use reporter::Message;
pub use reporter::Reporter;
pub use renderer::Renderer;


#[derive(Debug)]
pub struct Widget {
	rx: sync::mpsc::Receiver<Message>,
	status: Status,
	items: Vec<(Box<str>, ItemStatus)>,
	finished: bool,
}


impl Widget {
	pub fn new<WSError>() -> (Self, Reporter<WSError>)
	where
		WSError: Error,
	{
		let (tx, rx) = sync::mpsc::channel();

		(
			Self {
				rx,
				status: Default::default(),
				items: Default::default(),
				finished: false,
			},
			Reporter::new(tx)
		)
	}


	pub fn finished(&self) -> bool {
		self.finished
	}


	pub fn update(&mut self) {
		if self.finished() {
			return;
		}

		for item in self.rx.try_iter() {
			match item {
				Message::SizeHint(size, _) => self.items.resize_with(size, Default::default),

				Message::Item(id, label) => self.items[id as usize].0 = label,

				Message::ItemStatus(id, status) => self.items[id as usize].1 = status,

				Message::Status(status) => self.status = status,

				Message::Finish(status) => {
					self.status = status;
					self.finished = true;
				},
			};
		}
	}


	pub fn renderer(&self) -> Renderer {
		Renderer(self)
	}
}
