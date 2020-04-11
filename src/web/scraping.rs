pub use scraper::{ElementRef, Html, Selector};

use regex::Regex;

use thiserror::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum Error {
	#[error("element not found: {0}")]
	NotFound(Box<str>),

	#[error("empty element: {0}")]
	EmptyElement(Box<str>),

	#[error("format error: {0}")]
	Format(Box<str>),
}


pub enum Select<'a, 'b> {
	Html(scraper::html::Select<'a, 'b>),
	Element(scraper::element_ref::Select<'a, 'b>),
}


impl<'a, 'b> Iterator for Select<'a, 'b> {
	type Item = ElementRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Select::Html(select) => select.next(),
			Select::Element(select) => select.next(),
		}
	}
}


pub trait Selectable<'a> {
	fn select_iter<'b>(self, selector: &'b Selector) -> Select<'a, 'b>;
}


impl<'a> Selectable<'a> for &'a Html {
	fn select_iter<'b>(self, selector: &'b Selector) -> Select<'a, 'b> {
		Select::Html(
			self.select(selector)
		)
	}
}


impl<'a> Selectable<'a> for ElementRef<'a> {
	fn select_iter<'b>(self, selector: &'b Selector) -> Select<'a, 'b> {
		Select::Element(
			self.select(selector)
		)
	}
}


pub trait Find<'a> {
	fn find(self, selector: &str) -> Result<ElementRef<'a>, Error>;

	fn find_regex(self, selector: &str, regex: &str) -> Result<ElementRef<'a>, Error>;
}


impl<'a, T> Find<'a> for T
where
	T: Selectable<'a>
{
	fn find(self, selector: &str) -> Result<ElementRef<'a>, Error> {
		self
			.select_iter(
				&Selector::parse(selector)
					.expect("Invalid css selector")
			)
			.next()
			.ok_or(
				Error::NotFound(selector.into())
			)
	}


	fn find_regex(self, selector: &str, regex: &str) -> Result<ElementRef<'a>, Error> {
		let regex = Regex
			::new(regex)
			.expect("invalid regex");

		self
			.select_iter(
				&Selector::parse(selector)
					.expect("invalid css selector")
			)
			.find(
				|element| element
					.text_first()
					.map(
						|text| regex.is_match(text)
					)
					.unwrap_or(false)
			)
			.ok_or(
				Error::NotFound(selector.into())
			)
	}
}


pub trait Text<'a> {
	fn text_first(&self) -> Result<&'a str, Error>;
}


impl<'a> Text<'a> for ElementRef<'a> {
	fn text_first(&self) -> Result<&'a str, Error> {
		self
			.text()
			.next()
			.map(str::trim)
			.filter(
				|s| !s.is_empty()
			)
			.ok_or(
				Error::EmptyElement(
					"text".into()
				)
			)
	}
}


pub trait FindNext<'a> {
	fn find_next(&self, element: &str) -> Result<ElementRef<'a>, Error>;
}


impl<'a> FindNext<'a> for ElementRef<'a> {
	fn find_next(&self, name: &str) -> Result<ElementRef<'a>, Error> {
		let not_found = || Error::NotFound(
			name.into()
		);

		let element = ElementRef
			::wrap(
				self
					.next_siblings()
					.find(
						|node| node
							.value()
							.as_element()
							.map(
								|element| element.name() == name
							)
							.unwrap_or(false)
					)
					.ok_or_else(not_found)?
			)
			.ok_or_else(not_found)?;

		Ok(element)
	}
}
