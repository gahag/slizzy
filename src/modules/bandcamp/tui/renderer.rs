use std::borrow::Cow;

use tui::{
	buffer::Buffer,
	layout::{Rect, Corner},
	style::{Color, Style},
	widgets::{Block, Borders, Text, List, Widget},
};

use super::{ItemStatus, Status};


pub struct Renderer<'a>(pub &'a super::Widget);


impl<'a> Renderer<'a> {
	fn render_title(&self) -> (Cow<'a, str>, Style) {
		match &self.0.status {
			Status::Initializing => (
				" Bandcamp - initializing ".into(),
				Style::default()
			),

			Status::Skipped => (
				" Bandcamp - skipped ".into(),
				Style
					::default()
					.fg(Color::Green)
			),

			Status::Searching => (
				" Bandcamp - searching ".into(),
				Style::default()
			),

			Status::Done => (
				" Bandcamp - done ".into(),
				Style
					::default()
					.fg(Color::Green)
			),

			Status::MatchNotFound => (
				" Bandcamp - match not found ".into(),
				Style
					::default()
					.fg(Color::Yellow)
			),

			Status::Error(error) => (
				format!(" Bandcamp - {} ", error).into(),
				Style
					::default()
					.fg(Color::Red)
			),
		}
	}


	fn render_items(&self) -> impl Iterator<Item = Text<'a>> {
		self.0.items
			.iter()
			.map(
				|(label, status)| {
					let (label, style) = match status {
						ItemStatus::Fetching => (
							label.as_ref().into(),
							Style::default()
						),

						ItemStatus::Selected => (
							label.as_ref().into(),
							Style
								::default()
								.fg(Color::Green),
						),

						ItemStatus::Filtered => (
							format!("{} | not a track page", label)
								.into(),
							Style
								::default()
								.fg(Color::Yellow),
						),

						ItemStatus::TitleMismatch(sim) => (
							format!(
								"{} | title mismatch: {}% similarity below threshold",
								label,
								sim.value()
							)
								.into(),
							Style
								::default()
								.fg(Color::Yellow),
						),

						ItemStatus::Error(error) => (
							format!("{} | error: {}", label, error)
								.into(),
							Style
								::default()
								.fg(Color::Red),
						),
					};

					Text::Styled(label, style)
				}
			)
	}
}


impl<'a> Widget for Renderer<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let (title, title_style) = self.render_title();

		let items = self.render_items();

		let widget = List
			::new(items)
			.block(
				Block
					::default()
					.borders(Borders::ALL)
					.title(&title)
					.title_style(title_style)
			)
			.start_corner(Corner::TopLeft);

		widget.render(area, buf);
	}
}
