use serde::{de, Deserialize, Deserializer};


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

		Ok(
			Items(
				data.items
					.into_vec()
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
	items: Box<[Link]>
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Info {
	pub total_items: u8,
}


impl<'de> Deserialize<'de> for Info {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let info = SearchInfo::deserialize(deserializer)?;

		let info = Info {
			total_items: info.search_information.total_results
				.parse()
				.map_err(de::Error::custom)?
		};

		Ok(info)
	}
}


#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct SearchInfo<'a> {
	#[serde(borrow)]
	search_information: InnerInfo<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct InnerInfo<'a> {
	total_results: &'a str,
}
