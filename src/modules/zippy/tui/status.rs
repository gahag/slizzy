use std::error::Error;

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


impl<E: Error> From<&BackendStatus<E>> for Status {
	fn from(status: &BackendStatus<E>) -> Self {
		match status {
			BackendStatus::Fetching => Status::Fetching,

			BackendStatus::NoEntries => Status::NoEntries,

			BackendStatus::Error(BackendError::WSError(error)) => Status::Error(
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
	Expired,
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
			BackendItemStatus::Expired => ItemStatus::Expired,
			BackendItemStatus::Filtered(filter) => ItemStatus::Filtered(
				filter.clone() // unfortunately, we have to clone here.
			),
			BackendItemStatus::Downloading(progress) => ItemStatus::Downloading(*progress),
			BackendItemStatus::Done => ItemStatus::Done,
		}
	}
}
