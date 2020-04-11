mod renderer;
mod reporter;
mod status;

use std::sync;

use super::{
	super::item::progress,
	SourceError as BackendError,
	Status as BackendStatus,
	ItemStatus as BackendItemStatus,
	Filter,
};
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
	pub fn new() -> (Self, Reporter) {
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
