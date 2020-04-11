mod layout;

use std::io;
use std::time::Duration;
pub use std::thread::JoinHandle;

use super::{Backend, Key, StdinReader, Terminal};
use crate::util::future::abortable::AbortHandle;
pub use crate::modules::{
	beatport::tui::Widget as BeatportWidget,
	slider::tui::Widget as SliderWidget,
	zippy::tui::Widget as ZippyWidget,
	music2k::tui::Widget as Music2kWidget,
};


pub struct Window {
	pub aborter: AbortHandle,

	pub terminal: Terminal<Backend>,
	pub input: StdinReader,

	// Widgets:
	pub beatport_widget: BeatportWidget,
	pub slider_widget: SliderWidget,
	pub zippy_widget: ZippyWidget,
	pub music2k_widget: Music2kWidget,
}


impl Window {
	pub fn run(self) -> JoinHandle<io::Result<()>> {
		std::thread::Builder
			::new()
			.name(
				"slizzy ui".to_owned()
			)
			.spawn(
				move || self.run_loop()
			)
			.expect("failed to spawn thread")
	}


	fn update_widgets(&mut self) {
		self.beatport_widget.update();
		self.slider_widget.update();
		self.zippy_widget.update();
		self.music2k_widget.update();
	}


	fn draw(&mut self) -> io::Result<()> {
		let beatport_widget = self.beatport_widget.renderer();
		let slider_widget = self.slider_widget.renderer();
		let zippy_widget = self.zippy_widget.renderer();
		let music2k_widget = self.music2k_widget.renderer();

		self.terminal.draw(
			|mut frame| {
				let root_layout = layout::Root::new(frame.size());

				let tracksources_layout = layout::TrackSources::new(root_layout.tracksources);

				{
					frame.render_widget(beatport_widget, root_layout.metasources);
				}

				{
					frame.render_widget(slider_widget, tracksources_layout.slider);
					frame.render_widget(zippy_widget, tracksources_layout.zippy);
					frame.render_widget(music2k_widget, tracksources_layout.music2k);
				}
			}
		)
	}


	fn run_loop(mut self) -> io::Result<()> {
		self.terminal.hide_cursor()?;

		let mut quit = false;

		while !quit {
			for action in self.input.read_action() {
				match action? {
					Action::Quit => {
						self.aborter.abort();
						quit = true;
					},
					Action::Key(key) => {
						log::debug!("user input: {:?}", key);
						// TODO: forward to active widget
					},
				}
			};

			self.update_widgets();

			self.draw()?;

			std::thread::sleep(
				Duration::from_millis(300)
			);
		}

		self.terminal.show_cursor()?;

		Ok(())
	}
}


impl std::fmt::Debug for Window {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Params")
			.field("beatport_widget", &self.beatport_widget)
			.field("slider_widget", &self.slider_widget)
			.finish()
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
	Quit,
	Key(Key),
}


impl From<Key> for Action {
	fn from(key: Key) -> Self {
		match key {
			Key::Char('q') => Action::Quit,
			other => Action::Key(other),
		}
	}
}
