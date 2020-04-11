use std::marker::PhantomData;

use super::Progress;


#[derive(Debug, Clone, Copy, Default)]
pub struct NoProgress<Id, Item, ItemStatus, Status>
where
	Id: 'static,
	Item: 'static,
	ItemStatus: 'static,
	Status: 'static,
{
	id: PhantomData<&'static Id>,
	item: PhantomData<&'static Item>,
	item_status: PhantomData<&'static ItemStatus>,
	status: PhantomData<&'static Status>,
}


impl<Id, Item, ItemStatus, Status> NoProgress<Id, Item, ItemStatus, Status> {
	pub fn new() -> Self {
		NoProgress {
			id: PhantomData,
			item: PhantomData,
			item_status: PhantomData,
			status: PhantomData,
		}
	}
}


impl<Id, Item, ItemStatus, Status> Progress for NoProgress<Id, Item, ItemStatus, Status>
where
	Id: Eq + Ord + std::hash::Hash
{
	type Id = Id;
	type Item = Item;
	type Status = Status;
	type ItemStatus = ItemStatus;

	fn item(&self, _id: Self::Id, _item: &Self::Item) { }

	fn item_status(&self, _id: Self::Id, _status: &Self::ItemStatus) { }

	fn finish(&self, _status: &Self::Status) { }

	fn status(&self, _status: &Self::Status) { }
}
