use crate::util;
use super::{BackendError, BackendStatus, BackendItemStatus};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
	Initializing,
	Fetching,
	NoEntries,
	Error(Box<str>),
	Done,
}


impl Default for Status {
	fn default() -> Self { Status::Initializing }
}


impl From<&BackendStatus> for Status {
	fn from(status: &BackendStatus) -> Self {
		match status {
			BackendStatus::Fetching => Status::Fetching,

			BackendStatus::NoEntries => Status::NoEntries,

			BackendStatus::Error(BackendError::Http(error)) => Status::Error(
				error
					.to_string()
					.into_boxed_str()
			),

			BackendStatus::Error(BackendError::Items(_)) => Status::Error(
				"some items failed, check the log for details".into()
			),


			BackendStatus::Done => Status::Done,
		}
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemStatus {
	Fetching,
	Error(Box<str>),
	Filtered(super::Filter),
	Downloading(util::io::Progress),
	Done
}


impl Default for ItemStatus {
	fn default() -> Self { ItemStatus::Fetching }
}


impl From<&BackendItemStatus> for ItemStatus {
	fn from(status: &BackendItemStatus) -> Self {
		match status {
			BackendItemStatus::Error(error) => ItemStatus::Error(
				error
					.to_string()
					.into_boxed_str()
			),
			BackendItemStatus::Filtered(filter) => ItemStatus::Filtered(
				filter.clone() // unfortunately, we have to clone here.
			),
			BackendItemStatus::Downloading(progress) => ItemStatus::Downloading(*progress),
			BackendItemStatus::Done => ItemStatus::Done,
		}
	}
}
