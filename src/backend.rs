use enumset::EnumSet;

use futures::{
	future::{LocalBoxFuture, FutureExt},
	stream::{FuturesUnordered, StreamExt}
};

use crate::{
	track::Track,
	modules::{
		metasource::{Module as MetaSource, MetaSources},
		tracksource::{Module as TrackSource, TrackSources},
		beatport::SourceParams as BeatportParams,
		slider::SourceProgress as SliderParams,
		zippy::SourceParams as ZippyParams,
		music2k::SourceProgress as Music2kParams,
	},
	util::error::{self, AggregateError},
};
pub use crate::modules::{
	google::{
		Module as Google,
		SearchError as GoogleError,
	},
	beatport::{
		Module as Beatport,
		tui::Reporter as BeatportReporter,
	},
	slider::{
		Module as Slider,
		tui::Reporter as SliderReporter,
	},
	zippy::{
		Module as Zippy,
		tui::Reporter as ZippyReporter,
	},
	music2k::{
		Module as Music2k,
		tui::Reporter as Music2kReporter,
	},
};


#[derive(Debug)]
pub struct Backend {
	pub metasources: EnumSet<MetaSources>,
	pub tracksources: EnumSet<TrackSources>,

	pub google: Google,

	pub beatport: Beatport<Google>,
	pub beatport_reporter: BeatportReporter<GoogleError>,

	pub slider: Slider,
	pub slider_reporter: SliderReporter,

	pub zippy: Zippy<Google>,
	pub zippy_reporter: ZippyReporter<GoogleError>,

	pub music2k: Music2k,
	pub music2k_reporter: Music2kReporter,
}


impl Backend {
	pub async fn run(self, track: &mut Track) -> anyhow::Result<()> {
		{
			let mut success = false;

			if !success && self.metasources.contains(MetaSources::Beatport) {
				success = self.beatport
					.fill_metadata(
						track,
						BeatportParams {
							progress: Box::new(self.beatport_reporter),
							// We shouldn't have to clone here, but the params was designed to be owned.
							websearch: self.google.clone(),
						}
					)
					.await?;
			}

			if !success {
				anyhow::bail!("failed to fetch track duration");
			}
		}

		log::info!("downloading track: {}", track);

		{
			let track = &track;

			let tracksources_futures = FuturesUnordered::<LocalBoxFuture<anyhow::Result<()>>>::new();

			if self.tracksources.contains(TrackSources::Slider) {
				let params = SliderParams(
					Box::new(self.slider_reporter)
				);

				tracksources_futures.push(
					self.slider
						.fetch(&track, params)
						.map(error::anyhow_result)
						.boxed_local()
				);
			}

			if self.tracksources.contains(TrackSources::Zippy) {
				let params = ZippyParams {
					progress: Box::new(self.zippy_reporter),
					websearch: self.google,
				};

				tracksources_futures.push(
					self.zippy
						.fetch(&track, params)
						.map(error::anyhow_result)
						.boxed_local()
				);
			}

			if self.tracksources.contains(TrackSources::Music2k) {
				let params = Music2kParams(
					Box::new(self.music2k_reporter)
				);

				tracksources_futures.push(
					self.music2k
						.fetch(&track, params)
						.map(error::anyhow_result)
						.boxed_local()
				);
			}

			let errors: AggregateError = tracksources_futures
				.filter_map(
					|result| async move {
						result.err()
					}
				)
				.collect()
				.await;

			if errors.is_empty() {
				Ok(())
			}
			else {
				Err(
					errors.into()
				)
			}
		}
	}
}
