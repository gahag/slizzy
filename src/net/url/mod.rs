mod query;

use std::{
	borrow::{Borrow, Cow},
	convert::TryFrom,
	hash::Hash,
	marker::PhantomData,
	str::FromStr,
};

use derive_more::{AsRef, Display};

use thiserror::Error;

pub use query::Query;


#[derive(Debug, Display, Error)]
pub struct SchemeError;


#[derive(Debug, Display, Error)]
pub struct HostError;


#[derive(Debug, Display, Error)]
pub struct PathError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dissected<'a> {
	url: url::Url,
	// Just in case we want to reference extern strings in the future:
	_marker: PhantomData<&'a ()>,
}


impl<'a> Dissected<'a> {
	/// Get the URL scheme.
	pub fn scheme(&self) -> &str {
		self.url.scheme()
	}


	/// Sets the URL's scheme.
	pub fn set_scheme<S>(&mut self, scheme: S) -> Result<&mut Self, SchemeError>
	where
		S: AsRef<str>
	{
		let scheme = scheme.as_ref();

		self.url
			.set_scheme(scheme)
			.map_err(|_| SchemeError)?;

		Ok(self)
	}


	/// Gets the URL's host.
	pub fn host(&self) -> Option<&str> {
		self.url.host_str()
	}


	/// Sets the URL's host.
	pub fn set_host<S>(&mut self, host: S) -> Result<&mut Self, HostError>
	where
		S: AsRef<str>
	{
		let host = host.as_ref();

		self.url
			.set_host(Some(host))
			.map_err(|_| HostError)?;

		Ok(self)
	}


	/// Gets the path.
	pub fn path_str(&self) -> &str {
		self.url.path()
	}


	/// Gets an iterator to the path's components.
	pub fn path(&self) -> impl Iterator<Item = &str> {
		self.url
			.path_segments()
			.unwrap_or_else(
				|| "".split(' ')
			)
	}


	pub fn clear_path(&mut self) -> &mut Self {
		self.url.set_path("/");
		self
	}


	/// Extends the path with additional components.
	/// The components are considered to be relative to the current path, if any.
	pub fn extend_path<S>(
		&mut self,
		parts: impl IntoIterator<Item = S>
	) -> Result<&mut Self, PathError>
	where
		S: AsRef<str>
	{
		self.url
			.path_segments_mut()
			.map_err(|_| PathError)?
			.pop_if_empty()
			.extend(parts);

		Ok(self)
	}


	/// Push a path to the URL.
	/// If the given path is absolute (starts with '/'), the URL's path is dropped, if any.
	pub fn push_path<S>(&mut self, segment: S) -> Result<&mut Self, PathError>
	where
		S: AsRef<str>
	{
		let segments = segment.as_ref();

		if segments.starts_with('/') {
			self.clear_path();
		}

		self.extend_path(
			segments.split('/')
		)
	}


	pub fn query(&self) -> Query {
		Query::new(
			self.url.query_pairs()
		)
	}


	pub fn set_query(&mut self, new: Query) -> &mut Self {
		{
			let mut query = self.url.query_pairs_mut();

			query.clear();

			query.extend_pairs(
				new.iter()
			);
		}

		self
	}


	pub fn query_str(&self) -> Option<&str> {
		self.url.query()
	}


	pub fn query_pairs(&self) -> impl Iterator<Item = (Cow<str>, Cow<str>)> {
		self.url.query_pairs()
	}


	pub fn clear_query(&mut self) -> &mut Self {
		self.url
			.query_pairs_mut()
			.clear();

		self
	}


	pub fn append_query(&mut self, name: &str, value: &str) -> &mut Self {
		self.url
			.query_pairs_mut()
			.append_pair(name, value);

		self
	}


	pub fn extend_query<I, K, V>(&mut self, iter: I) -> &mut Self
	where
		I: IntoIterator,
		I::Item: Borrow<(K, V)>,
		K: AsRef<str>,
		V: AsRef<str>,
	{
		self.url
			.query_pairs_mut()
			.extend_pairs(iter);

		self
	}


	pub fn fragment(&self) -> Option<&str> {
		self.url.fragment()
	}


	pub fn set_fragment<S>(&mut self, fragment: S) -> &mut Self
	where
		S: AsRef<str>
	{
		let fragment = fragment.as_ref();

		self.url.set_fragment(Some(fragment));

		self
	}


	pub fn assemble(&self) -> Url {
		// In the ideal design, the url would actually be assembled here, instead of cloned.
		Url(
			self.url.clone()
		)
	}
}


impl<'a> TryFrom<&'a str> for Dissected<'a> {
	type Error = url::ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let url = value.parse()?;

		Ok(
			Self {
				url,
				_marker: PhantomData
			}
		)
	}
}


#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, AsRef)]
#[as_ref(forward)]
pub struct Url(url::Url);


impl Url {
	pub fn scheme(&self) -> &str {
		self.0.scheme()
	}


	pub fn host(&self) -> Option<&str> {
		self.0.host_str()
	}


	pub fn dissect(&self) -> Dissected {
		Dissected {
			// In the ideal design, the dissected would refer this url's string instead of
			// cloning.
			url: self.0.clone(),
			_marker: PhantomData
		}
	}
}


impl FromStr for Url {
	type Err = url::ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let url = s.parse()?;

		Ok(
			Self(url)
		)
	}
}


impl TryFrom<&str> for Url {
	type Error = url::ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_push_path() {
		let test = |base, path, expected| {
			let mut base = Dissected
				::try_from(base)
				.expect("invalid base url");

			base
				.push_path(path)
				.expect("failed to push path");

			assert_eq!(
				base.assemble().as_ref(),
				expected
			);
		};

		test(
			"https://www5.zippyshare.com/v/422Nyysq/file.html",
			"/downloadMusicHQ",
			"https://www5.zippyshare.com/downloadMusicHQ"
		);

		test(
			"https://www5.zippyshare.com/v/422Nyysq/file.html",
			"downloadMusicHQ",
			"https://www5.zippyshare.com/v/422Nyysq/file.html/downloadMusicHQ"
		);

		test(
			"https://www5.zippyshare.com/v/422Nyysq/file.html/",
			"downloadMusicHQ/test.csv",
			"https://www5.zippyshare.com/v/422Nyysq/file.html/downloadMusicHQ/test.csv"
		);
	}
}
