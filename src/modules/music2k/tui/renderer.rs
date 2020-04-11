use std::borrow::Cow;

use tui::{
	buffer::Buffer,
	layout::{Rect, Corner},
	style::{Color, Style},
	widgets::{Block, Borders, Text, List, Widget},
};

use crate::util::bytes;
use super::{ItemStatus, Status, Filter};


pub struct Renderer<'a>(pub &'a super::Widget);


impl<'a> Renderer<'a> {
	fn render_title(&self) -> (Cow<'a, str>, Style) {
		match &self.0.status {
			Status::Initializing => (
				" Music2k - initializing ".into(),
				Style
					::default()
					.fg(Color::Gray)
			),

			Status::Fetching => (
				" Music2k - fetching ".into(),
				Style::default()
			),

			Status::NoEntries => (
				" Music2k - no entries found ".into(),
				Style::default()
			),

			Status::Error(error) => (
				format!(" Music2k - {} ", error).into(),
				Style
					::default()
					.fg(Color::Red)
			),

			Status::Done => (
				" Music2k - done ".into(),
				Style
					::default()
					.fg(Color::Green)
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

						ItemStatus::Error(error) => (
							format!("{} | error: {}", label, error)
								.into(),
							Style
								::default()
								.fg(Color::Red),
						),

						ItemStatus::Filtered(filter) => {
							let text = match filter {
								Filter::Id(sim) => format!(
									"{} | id mismatch: {}% similarity below threshold",
									label,
									sim.value()
								),

								Filter::Duration(duration) => format!(
									"{} | duration mismatch: {} out of range",
									label,
									duration
								),

								Filter::Size(size) => format!(
									"{} | file size mismatch: {} out of range",
									label,
									size
								),
							};

							(
								text.into(),
								Style
									::default()
									.fg(Color::Yellow),
							)
						},

						ItemStatus::Downloading(progress) => {
							let text =
								if let Some(percentage) = progress.percentage() {
									let percentage = percentage * 100.0;

									format!("{} | downloading: {:.1}%", label, percentage)
										.into()
								}
								else {
									let megabytes: bytes::Mb = progress.completed.into();

									format!("{} | downloading: {}", label, megabytes)
										.into()
								};

							(
								text,
								Style
									::default()
									.fg(Color::LightBlue),
							)
						},

						ItemStatus::Done => (
							format!("{} | done!", label)
								.into(),
							Style
								::default()
								.fg(Color::Green),
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
