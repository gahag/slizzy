mod noprogress;

pub use noprogress::NoProgress;


pub trait Progress {
	type Id: Eq + Ord + std::hash::Hash;
	type Item: ?Sized;
	type Status: ?Sized;
	type ItemStatus: ?Sized;


	fn size_hint(&self, _hint: (usize, Option<usize>)) { }

	fn item(&self, id: Self::Id, item: &Self::Item);

	fn item_status(&self, id: Self::Id, status: &Self::ItemStatus);

	fn status(&self, status: &Self::Status);

	fn finish(&self, status: &Self::Status);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Message<Id, Item, Status, ItemStatus> {
	SizeHint(usize, Option<usize>),
	Item(Id, Item),
	ItemStatus(Id, ItemStatus),
	Status(Status),
	Finish(Status),
}
