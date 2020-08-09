use serde::{Deserialize, Deserializer};

#[cfg(test)]
mod tests;


// We won't parse the URLs here to allow fault tolerance:
// A single malformed URL won't cause a generalized failure.
pub type Item = Box<str>;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Items(pub Box<[Item]>);


impl IntoIterator for Items {
	type Item = Item;
	type IntoIter = <Vec<Item> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0
			.into_vec()
			.into_iter()
	}
}


impl<'de> Deserialize<'de> for Items {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let data = SearchItems::deserialize(deserializer)?;

		let items = match data.items {
			None => Vec::new(),
			Some(items) => items.into_vec(),
		};

		Ok(
			Items(
				items
					.into_iter()
					.map(
						|i| i.link
					)
					.collect()
			)
		)
	}
}


#[derive(Debug, Deserialize)]
struct Link {
	link: Item,
}

#[derive(Debug, Deserialize)]
struct SearchItems {
	items: Option<Box<[Link]>>
}
