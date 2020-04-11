use std::{
	borrow::{Borrow, Cow},
	collections::HashMap,
	hash::Hash,
};


/// A HashMap like structure for accessing query parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query<'a>(
	HashMap<
		Cow<'a, str>,
		Cow<'a, str>
	>
);


impl<'a> Query<'a> {
	pub fn new<Q>(query: Q) -> Query<'a>
	where
		Q: IntoIterator<Item = (Cow<'a, str>, Cow<'a, str>)>
	{
		Query(
			query
				.into_iter()
				.collect()
		)
	}


	pub fn clear(&mut self) {
		self.0.clear()
	}


	/// Get an iterator to the keys.
	pub fn keys(&self) -> impl Iterator<Item = &str> {
		self.0
			.keys()
			.map(AsRef::as_ref)
	}


	/// Get an iterator to the values.
	pub fn values(&self) -> impl Iterator<Item = &str> {
		self.0
			.values()
			.map(AsRef::as_ref)
	}


	/// Get an iterator to the key-value pairs.
	pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
		self.0
			.iter()
			.map(
				|(k, v)| (k.as_ref(), v.as_ref())
			)
	}


	/// How many parameters there are.
	pub fn len(&self) -> usize {
		self.0.len()
	}


	/// Is there any parameter?
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}


	/// Check if the given parameter is present.
	pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
	where
		Cow<'a, str>: Borrow<Q>,
		Q: Hash + Eq,
	{
		self.0.contains_key(k)
	}


	/// Get the value for a given parameter.
	pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&str>
	where
		Cow<'a, str>: Borrow<Q>,
		Q: Hash + Eq,
	{
		self.0
			.get(k)
			.map(AsRef::as_ref)
	}


	pub fn get_mut<K: ?Sized>(&mut self, k: &K) -> Option<&mut String>
	where
		Cow<'a, str>: Borrow<K>,
		K: Hash + Eq,
	{
		self.0
			.get_mut(k)
			.map(Cow::to_mut)
	}


	pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<Cow<str>>
	where
		K: Into<Cow<'a, str>>,
		V: Into<Cow<'a, str>>
	{
		self.0.insert(
			k.into(),
			v.into()
		)
	}


	pub fn remove<K: ?Sized>(&mut self, k: &K) -> Option<Cow<str>>
	where
		K: Into<Cow<'a, str>> + Hash + Eq,
		Cow<'a, str>: Borrow<K>,
	{
		self.0.remove(
			k.into()
		)
	}
}


impl<'a, K, V> Extend<(K, V)> for Query<'a>
where
	K: Into<Cow<'a, str>>,
	V: Into<Cow<'a, str>>
{
	fn extend<T>(&mut self, items: T)
	where
		T: IntoIterator<Item = (K, V)>
	{
		self.0.extend(
			items
				.into_iter()
				.map(
					|(k, v)| (k.into(), v.into())
				)
		)
	}
}
