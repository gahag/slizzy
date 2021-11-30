use super::*;
use crate::track::Duration;


#[test]
fn test_valid() {
	let payload = std::include_str!("test.json");

	let json = serde_json
		::from_str::<Json>(payload)
		.expect("failed to parse json");

	let expected = Json {
		audios: AudiosObject(
			Box::new(
				[
					Entry {
						id: "371745443_456552853".into(),
						url: "psv4/c815137/u371745443/audios/864ffa53ed2b".into(),
						track_id: "Somne, Mind Against - Vertere".into(),
						duration: Duration::new(9, 14),
						extra: Some("IUZ-ACRuwtNix4E8bePkI20u2owgeJBf2NFP-ZFwymXCD-fvNnHK3ixzxcWsbhwNWFu0seQBNGG-_aF5-iGy1jdKeRNrsblQ4rZivQmF_sxxUTgUJyarOgprjpTO-wrfMrNS-MWJTp-lS93cUpDFM-0".into()),
					},
					Entry {
						id: "-2001463066_59463066".into(),
						url: "cs1-41v4/p1/844e0fb9227745".into(),
						track_id: "Georgi Z - Vertere".into(),
						duration: Duration::new(6, 24),
						extra: Some("Iwm_p382q0AB06-n22zGWZ4Oud7cJsPEJt-hvlQa-hrn55sZJ7-VXtB8gxZykPjifMtc6cKADbRPfgmFup05DbZ6g2olAb7wbDGjO8ksaGZCirP3RHeu48eFUOq7yXB_Db-XHaVw_vb3GtVnAw91U8k5".into()),
					},
				],
			)
		)
	};

	assert_eq!(json, expected);
}
