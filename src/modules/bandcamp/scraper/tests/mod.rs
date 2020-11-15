use super::*;

use crate::{
	track::Duration,
	web::scraping,
};


#[test]
fn test_scrap() {
	let doc = include_str!("page.html");

	let doc = scraping::Html::parse_document(&doc);

	let data = scrap(&doc);

	assert_eq!(
		data,
		Data {
			track: Ok("Bolam - Who Knows".into()),
			duration: Ok(Duration::new(8, 41)),
		}
	);
}
