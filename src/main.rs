#![allow(dead_code)]

mod args;
mod backend;
mod config;
mod net;
mod track;
mod util;
mod web;
mod sim;
mod modules;
mod tui;
mod logger;

use std::io;

use crate::{
	args::{Args, Command},
	modules::Module,
	util::future::abortable::{Aborted, abortable},
};


fn main() -> ! {
	let exit_code = match run() {
		Ok(_) => 0,
		Err(error) => {
			eprintln!("Error: {}", error);
			1
		}
	};

	std::process::exit(exit_code)
}


fn run() -> anyhow::Result<()> {
	let command = args
		::parse(
			std::env::args_os()
		)
		.map_err(anyhow::Error::new)?;

	match command {
		Command::Help(msg) | Command::Version(msg) => {
			println!("{}", msg);
			Ok(())
		},
		Command::Download(args) => download(args),
	}
}


fn download(args: Args) -> anyhow::Result<()> {
	let logger = logger::setup(args.log_level);

	let cfg = match config::load() {
		Err(error) if error.is_not_found() => {
			eprintln!(
				concat!(
					"The config file was not found.",
					" A template was placed, please edit it with your keys."
				)
			);

			std::process::exit(1)
		},

		other => other,
	}?;

	let mut track = args.track;

	let google_cfg = config::read(&cfg)?;
	let beatport_cfg = config::read(&cfg)?;
	let bandcamp_cfg = config::read(&cfg)?;
	let slider_cfg = config::read(&cfg)?;
	let zippy_cfg = config::read(&cfg)?;
	let music2k_cfg = config::read(&cfg)?;

	log::debug!("google cfg: {:#?}", google_cfg);
	log::debug!("beatport cfg: {:#?}", beatport_cfg);
	log::debug!("bandcamp cfg: {:#?}", bandcamp_cfg);
	log::debug!("slider cfg: {:#?}", slider_cfg);
	log::debug!("zippy cfg: {:#?}", slider_cfg);
	log::debug!("music2k cfg: {:#?}", slider_cfg);

	let google = modules::google::Module::new(google_cfg);
	let beatport = modules::beatport::Module::new(beatport_cfg);
	let bandcamp = modules::bandcamp::Module::new(bandcamp_cfg);
	let slider = modules::slider::Module::new(slider_cfg);
	let zippy = modules::zippy::Module::new(zippy_cfg);
	let music2k = modules::music2k::Module::new(music2k_cfg);

	let terminal = tui::terminal()?;
	let input = tui::StdinReader::new();

	let (beatport_widget, beatport_reporter) = modules::beatport::tui::Widget::new();
	let (bandcamp_widget, bandcamp_reporter) = modules::bandcamp::tui::Widget::new();
	let (slider_widget, slider_reporter) = modules::slider::tui::Widget::new();
	let (zippy_widget, zippy_reporter) = modules::zippy::tui::Widget::new();
	let (music2k_widget, music2k_reporter) = modules::music2k::tui::Widget::new();

	let backend = backend::Backend {
		metasources: args.metasources,
		tracksources: args.tracksources,

		google,

		beatport,
		beatport_reporter,

		bandcamp,
		bandcamp_reporter,

		slider,
		slider_reporter,

		zippy,
		zippy_reporter,

		music2k,
		music2k_reporter,
	};

	let (backend_fut, backend_abort) = abortable(
		backend.run(&mut track)
	);

	let window = tui::main::Window {
		aborter: backend_abort,
		terminal,
		input,
		beatport_widget,
		bandcamp_widget,
		slider_widget,
		zippy_widget,
		music2k_widget,
	};

	let ui_handle = window.run();

	let result = futures::executor::block_on(backend_fut);

	ui_handle
		.join()
		.expect("ui thread failed to join")?;

	let result = match result {
		Ok(result) => result,
		Err(Aborted) => {
			log::info!("slizzy aborted");
			Ok(())
		}
	};

	logger.dump(
		io::stdout().lock()
	)?;

	result
}
