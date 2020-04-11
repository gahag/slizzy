use regex::{RegexSet, SetMatches, SetMatchesIntoIter};


pub struct RegexSetMatches<'a> {
	iter: SetMatchesIntoIter,
	patterns: &'a[String],
}


impl<'a> RegexSetMatches<'a> {
	pub fn new(set: &'a RegexSet, matches: SetMatches) -> Self {
		Self {
			iter: matches.into_iter(),
			patterns: set.patterns(),
		}
	}
}


impl<'a> Iterator for RegexSetMatches<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		let ix = self.iter.next()?;

		Some(
			self.patterns[ix].as_ref()
		)
	}
}
