use super::*;

use std::convert::TryInto;

use crate::web::scraping;


#[test]
fn test_scrap() {
	let doc = include_str!("page.html");

	let doc = scraping::Html::parse_document(&doc);

	let url = "https://www111.zippyshare.com/v/kctFOhgh/file.html"
		.try_into()
		.expect("invalid source url");

	let data = scrap(&doc, &url);

	let download_url = "https://www111.zippyshare.com/d/kctFOhgh/1643035/Mind%2520Against%2520-%2520Walking%2520Away%2520%2528Original%2520Mix%2529%2520%255bwww.jkmk.net%255d.wav"
		.try_into()
		.expect("invalid download url");

	let track_id = "Mind Against - Walking Away (Original Mix) [www.jkmk.net].wav".into();

	let preview_url = "https://www111.zippyshare.com/downloadMusicHQ?key=kctFOhgh"
		.try_into()
		.expect("invalid preview url");

	match data {
		Data::Expired => panic!("data should not be expired"),
		Data::Available { download, metadata: Metadata { id, size, preview } } => {
			assert_eq!(download, Ok(download_url));
			assert_eq!(id, Ok(Some(track_id)));
			assert_eq!(size, Ok(75130472));
			assert_eq!(preview, Ok(preview_url));
		}
	}
}
