use std::future::Future;

use futures::future::Abortable;
pub use futures::future::{Aborted, AbortHandle};


pub fn abortable<F, T>(future: F) -> (
	impl Future<Output = Result<T, Aborted>>,
	AbortHandle
)
where
	F: Future<Output = T>
{
	let (abort_handle, abort_registration) = AbortHandle::new_pair();

	let future = Abortable::new(future, abort_registration);

	(future, abort_handle)
}
