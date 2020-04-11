use std::error::Error;

use crate::sim::Sim;
use super::{BackendStatus, BackendItemStatus};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
	Initializing,
	Skipped,
	Searching,
	Done,
	MatchNotFound,
	Error(Box<str>),
}


impl Default for Status {
	fn default() -> Self { Status::Initializing }
}


impl<E: Error> From<&BackendStatus<E>> for Status {
	fn from(status: &BackendStatus<E>) -> Self {
		match status {
			BackendStatus::Skipped => Status::Skipped,
			BackendStatus::Searching => Status::Searching,
			BackendStatus::Done => Status::Done,
			BackendStatus::MatchNotFound => Status::MatchNotFound,
			BackendStatus::Error(error) => Status::Error(
				error
					.to_string()
					.into_boxed_str()
			),
		}
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemStatus {
	Fetching,
	Selected,
	TitleMismatch(Sim),
	Error(Box<str>),
}


impl Default for ItemStatus {
	fn default() -> Self { ItemStatus::Fetching }
}


impl From<&BackendItemStatus> for ItemStatus {
	fn from(status: &BackendItemStatus) -> Self {
		match status {
			BackendItemStatus::Selected => ItemStatus::Selected,
			BackendItemStatus::TitleMismatch(sim) => ItemStatus::TitleMismatch(*sim),
			BackendItemStatus::Error(error) => ItemStatus::Error(
				error
					.to_string()
					.into_boxed_str()
			),
		}
	}
}
