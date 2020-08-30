use tui::layout::{Layout, Direction, Constraint, Rect};


pub struct Root {
	pub metasources: Rect,
	pub tracksources: Rect,
}


impl Root {
	pub fn new(base: Rect) -> Self {
		let chunks = Layout
			::default()
			.direction(Direction::Vertical)
			.constraints(
				vec![
					Constraint::Length(12), // 10 entries + 2 borders
					Constraint::Min(5)
				]
			)
			.split(base);

		match chunks.as_slice() {
			[top, bottom] => Self {
				metasources: *top,
				tracksources: *bottom,
			},
			_ => panic!("chunks pattern not matched"),
		}
	}
}


pub struct MetaSources {
	pub beatport: Rect,
	pub bandcamp: Rect,
}


impl MetaSources {
	pub fn new(base: Rect) -> Self {
		let chunks = Layout
			::default()
			.direction(Direction::Horizontal)
			.constraints(
				vec![
					Constraint::Percentage(50),
					Constraint::Percentage(50),
				]
			)
			.split(base);

		let (left, right) = match chunks.as_slice() {
			[left, right] => (*left, *right),
			_ => panic!("chunks pattern not matched"),
		};

		Self {
			beatport: left,
			bandcamp: right,
		}
	}
}


pub struct TrackSources {
	pub slider: Rect,
	pub zippy: Rect,
	pub music2k: Rect,
}


impl TrackSources {
	pub fn new(base: Rect) -> Self {
		let chunks = Layout
			::default()
			.direction(Direction::Horizontal)
			.constraints(
				vec![
					Constraint::Percentage(50),
					Constraint::Percentage(50),
				]
			)
			.split(base);

		let (left, right) = match chunks.as_slice() {
			[left, right] => (*left, *right),
			_ => panic!("chunks pattern not matched"),
		};

		let chunks = Layout
			::default()
			.direction(Direction::Vertical)
			.constraints(
				vec![
					Constraint::Percentage(50),
					Constraint::Percentage(50),
				]
			)
			.split(right);

		let (top_right, bottom_right) = match chunks.as_slice() {
			[top_right, bottom_right] => (*top_right, *bottom_right),
			_ => panic!("chunks pattern not matched"),
		};

		Self {
			slider: left,
			zippy: top_right,
			music2k: bottom_right,
		}
	}
}
