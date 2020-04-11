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
				" Slider - initializing ".into(),
				Style
					::default()
					.fg(Color::Gray)
			),

			Status::Fetching => (
				" Slider - fetching ".into(),
				Style::default()
			),

			Status::Attempt { number, error } => (
				format!(" Slider - attempt #{}: {} ", number, error).into(),
				Style
					::default()
					.fg(Color::Yellow)
			),

			Status::Error(error) => (
				format!(" Slider - {} ", error).into(),
				Style
					::default()
					.fg(Color::Red)
			),

			Status::Done => (
				" Slider - done ".into(),
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

						ItemStatus::Attempt { number, error } => (
							format!("{} | attempt #{}: {}", label, number, error)
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

						ItemStatus::Filtered(filter) => {
							let text = match filter {
								Filter::Id(sim) => format!(
									"{} | id mismatch: {}% similarity below threshold",
									label,
									sim.value()
								),

								Filter::Bitrate(bitrate) => format!(
									"{} | bitrate mismatch: {} out of range",
									label,
									bitrate
								),

								Filter::Duration(duration) => format!(
									"{} | duration mismatch: {} out of range",
									label,
									duration
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
