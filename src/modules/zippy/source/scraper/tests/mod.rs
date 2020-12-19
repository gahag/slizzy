use super::*;

use std::convert::TryInto;

use crate::web::scraping;


#[test]
fn test_scrap() {
	let doc = include_str!("page.html");

	let doc = scraping::Html::parse_document(&doc);

	let url = "https://www96.zippyshare.com/v/qRdAx7uY/file.html"
		.try_into()
		.expect("invalid source url");

	let data = scrap(&doc, &url);

	let download_url = "https://www96.zippyshare.com/d/qRdAx7uY/41820/In%2520Verruf%2520-%2520GHS06%2520%2528Original%2520Mix%2529%2520-%2520trax4mix.xyz.mp3"
		.try_into()
		.expect("invalid download url");

	let track_id = "In Verruf - GHS06 (Original Mix) - trax4mix.xyz.mp3".into();

	let preview_url = "https://www96.zippyshare.com/downloadMusicHQ?key=qRdAx7uY"
		.try_into()
		.expect("invalid preview url");

	match data {
		Data::Expired => panic!("data should not be expired"),
		Data::Available { download, metadata: Metadata { id, size, preview } } => {
			assert_eq!(download, Ok(download_url));
			assert_eq!(id, Ok(Some(track_id)));
			assert_eq!(size, Ok(14858322));
			assert_eq!(preview, Ok(preview_url));
		}
	}
}
