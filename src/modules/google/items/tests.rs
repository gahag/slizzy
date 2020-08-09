use super::*;


fn json_playload(payload: serde_json::Value) -> Items {
	serde_json
		::from_value(payload)
		.expect("invalid test payload")
}


#[test]
fn test_empty() {
	let payload = serde_json::json!({
		"kind": "customsearch#search",
		"url": {
			"type": "application/json",
			"template": "https://www.googleapis.com/customsearch/v1?q={searchTerms}&num={count?}&start={startIndex?}&lr={language?}&safe={safe?}&cx={cx?}&sort={sort?}&filter={filter?}&gl={gl?}&cr={cr?}&googlehost={googleHost?}&c2coff={disableCnTwTranslation?}&hq={hq?}&hl={hl?}&siteSearch={siteSearch?}&siteSearchFilter={siteSearchFilter?}&exactTerms={exactTerms?}&excludeTerms={excludeTerms?}&linkSite={linkSite?}&orTerms={orTerms?}&relatedSite={relatedSite?}&dateRestrict={dateRestrict?}&lowRange={lowRange?}&highRange={highRange?}&searchType={searchType}&fileType={fileType?}&rights={rights?}&imgSize={imgSize?}&imgType={imgType?}&imgColorType={imgColorType?}&imgDominantColor={imgDominantColor?}&alt=json"
		},
		"queries": {
			"request": [
				{
					"title": "Google Custom Search - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaasdkodskdoskdsoksd",
					"searchTerms": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaasdkodskdoskdsoksd",
					"count": 10,
					"startIndex": 1,
					"inputEncoding": "utf8",
					"outputEncoding": "utf8",
					"safe": "off",
					"cx": "<cx>"
				}
			]
		},
		"searchInformation": {
			"searchTime": 0.277615,
			"formattedSearchTime": "0.28",
			"totalResults": "0",
			"formattedTotalResults": "0"
		},
		"spelling": {
			"correctedQuery": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa sd kod sk dos kds oksd",
			"htmlCorrectedQuery": "<b><i>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa sd kod sk dos kds oksd</i></b>"
		}
	});

	assert_eq!(
		json_playload(payload),
		Items(Box::new([])),
	);
}


#[test]
fn test_results() {
	let payload: Items = serde_json
		::from_str(
			r####"
				{
					"kind": "customsearch#search",
					"url": {
						"type": "application/json",
						"template": "https://www.googleapis.com/customsearch/v1?q={searchTerms}&num={count?}&start={startIndex?}&lr={language?}&safe={safe?}&cx={cx?}&sort={sort?}&filter={filter?}&gl={gl?}&cr={cr?}&googlehost={googleHost?}&c2coff={disableCnTwTranslation?}&hq={hq?}&hl={hl?}&siteSearch={siteSearch?}&siteSearchFilter={siteSearchFilter?}&exactTerms={exactTerms?}&excludeTerms={excludeTerms?}&linkSite={linkSite?}&orTerms={orTerms?}&relatedSite={relatedSite?}&dateRestrict={dateRestrict?}&lowRange={lowRange?}&highRange={highRange?}&searchType={searchType}&fileType={fileType?}&rights={rights?}&imgSize={imgSize?}&imgType={imgType?}&imgColorType={imgColorType?}&imgDominantColor={imgDominantColor?}&alt=json"
					},
					"queries": {
						"request": [
							{
								"title": "Google Custom Search - vertere",
								"totalResults": "1340",
								"searchTerms": "vertere",
								"count": 10,
								"startIndex": 1,
								"inputEncoding": "utf8",
								"outputEncoding": "utf8",
								"safe": "off",
								"cx": "<cx>"
							}
						],
						"nextPage": [
							{
								"title": "Google Custom Search - vertere",
								"totalResults": "1340",
								"searchTerms": "vertere",
								"count": 10,
								"startIndex": 11,
								"inputEncoding": "utf8",
								"outputEncoding": "utf8",
								"safe": "off",
								"cx": "<cx>"
							}
						]
					},
					"context": {
						"title": "beatport"
					},
					"searchInformation": {
						"searchTime": 0.645516,
						"formattedSearchTime": "0.65",
						"totalResults": "1340",
						"formattedTotalResults": "1,340"
					},
					"items": [
						{
							"kind": "customsearch#result",
							"title": "Vertere (Original Mix) by Mind Against, Somne on Beatport",
							"htmlTitle": "\u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Mind Against, Somne on Beatport",
							"link": "https://www.beatport.com/track/vertere-original-mix/6755315",
							"displayLink": "www.beatport.com",
							"snippet": "Vertere. Original Mix. $1.29. Link: Embed: Artists Mind Against, Somne. Release. \n$2.58. Length 9:14; Released 2015-06-29; BPM 120; Key A♭ maj; Genre ...",
							"htmlSnippet": "\u003cb\u003eVertere\u003c/b\u003e. Original Mix. $1.29. Link: Embed: Artists Mind Against, Somne. Release. \u003cbr\u003e\n$2.58. Length 9:14; Released 2015-06-29; BPM 120; Key A♭ maj; Genre&nbsp;...",
							"cacheId": "g2fTeYFF4FMJ",
							"formattedUrl": "https://www.beatport.com/track/vertere-original-mix/6755315",
							"htmlFormattedUrl": "https://www.beatport.com/track/\u003cb\u003evertere\u003c/b\u003e-original-mix/6755315",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"og:type": "website",
										"twitter:card": "player",
										"twitter:title": "Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Download Now on Beatport.",
										"twitter:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"twitter:player": "https://embed.beatport.com/?id=6755315&type=track",
										"twitter:player:height": "165",
										"twitter:site": "@beatport",
										"twitter:player:stream:content_type": "audio/mpeg",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:player:stream": "https://geo-samples.beatport.com/track/981f159f-a376-4eac-b9b5-921e3cbc83f5.LOFI.mp3",
										"twitter:description": "Download Now on Beatport.",
										"og:url": "https://www.beatport.com/track/vertere-original-mix/6755315",
										"twitter:player:width": "480"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Vertere from Life And Death on Beatport",
							"htmlTitle": "\u003cb\u003eVertere\u003c/b\u003e from Life And Death on Beatport",
							"link": "https://www.beatport.com/release/vertere/1549121",
							"displayLink": "www.beatport.com",
							"snippet": "Jun 29, 2015 ... Constantly turning around itself, the winding melody of Vertere turns outward and \ninward at the same time, bringing the listener on a journey ...",
							"htmlSnippet": "Jun 29, 2015 \u003cb\u003e...\u003c/b\u003e Constantly turning around itself, the winding melody of \u003cb\u003eVertere\u003c/b\u003e turns outward and \u003cbr\u003e\ninward at the same time, bringing the listener on a journey&nbsp;...",
							"cacheId": "PJZYvfMopRUJ",
							"formattedUrl": "https://www.beatport.com/release/vertere/1549121",
							"htmlFormattedUrl": "https://www.beatport.com/release/\u003cb\u003evertere\u003c/b\u003e/1549121",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"og:type": "website",
										"twitter:card": "summary_large_image",
										"twitter:title": "Vertere from Life And Death on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Vertere from Life And Death on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Constantly turning around itself, the winding melody of Vertere turns outward and inward at the same time, bringing the listener on a journey outward while mirroring a simultaneous labyrinthine journey inward. Like the word itself, Vertere, with its roots in the Latin word Verto (which itself ...",
										"twitter:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"twitter:image:alt": "Vertere from Life And Death on Beatport",
										"twitter:site": "@beatport",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:description": "Constantly turning around itself, the winding melody of Vertere turns outward and inward at the same time, bringing the listener on a journey outward while ...",
										"og:url": "https://www.beatport.com/release/vertere/1549121"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Vertere (Original Mix) by Georgi Z on Beatport",
							"htmlTitle": "\u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Georgi Z on Beatport",
							"link": "https://www.beatport.com/track/vertere-original-mix/12797583",
							"displayLink": "www.beatport.com",
							"snippet": "Vertere. Original Mix. $1.29. Link: Embed: Artists Georgi Z. Release. $2.58. \nLength 6:24; Released 2019-12-06; BPM 124; Key G min; Genre Minimal / Deep\n ...",
							"htmlSnippet": "\u003cb\u003eVertere\u003c/b\u003e. Original Mix. $1.29. Link: Embed: Artists Georgi Z. Release. $2.58. \u003cbr\u003e\nLength 6:24; Released 2019-12-06; BPM 124; Key G min; Genre Minimal / Deep\u003cbr\u003e\n&nbsp;...",
							"cacheId": "XUhyXeXcBlUJ",
							"formattedUrl": "https://www.beatport.com/track/vertere-original-mix/12797583",
							"htmlFormattedUrl": "https://www.beatport.com/track/\u003cb\u003evertere\u003c/b\u003e-original-mix/12797583",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"og:type": "website",
										"twitter:card": "player",
										"twitter:title": "Vertere (Original Mix) by Georgi Z on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Vertere (Original Mix) by Georgi Z on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Download Now on Beatport.",
										"twitter:image": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"twitter:player": "https://embed.beatport.com/?id=12797583&type=track",
										"twitter:player:height": "165",
										"twitter:site": "@beatport",
										"twitter:player:stream:content_type": "audio/mpeg",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:player:stream": "https://geo-samples.beatport.com/track/34f83b4f-fd8c-497b-956a-a7e4e5cd050c.LOFI.mp3",
										"twitter:description": "Download Now on Beatport.",
										"og:url": "https://www.beatport.com/track/vertere-original-mix/12797583",
										"twitter:player:width": "480"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Vertere Chart by Mind Against: Tracks on Beatport",
							"htmlTitle": "\u003cb\u003eVertere\u003c/b\u003e Chart by Mind Against: Tracks on Beatport",
							"link": "https://www.beatport.com/chart/vertere-chart/356883",
							"displayLink": "www.beatport.com",
							"snippet": "Jun 30, 2015 ... Vertere Chart by Mind Against: Tracks on Beatport.",
							"htmlSnippet": "Jun 30, 2015 \u003cb\u003e...\u003c/b\u003e \u003cb\u003eVertere\u003c/b\u003e Chart by Mind Against: Tracks on Beatport.",
							"cacheId": "uzBnMnoHFy0J",
							"formattedUrl": "https://www.beatport.com/chart/vertere-chart/356883",
							"htmlFormattedUrl": "https://www.beatport.com/chart/\u003cb\u003evertere\u003c/b\u003e-chart/356883",
							"pagemap": {
								"cse_thumbnail": [
									{
										"src": "https://encrypted-tbn2.gstatic.com/images?q=tbn:ANd9GcSFxkB1pIruxtZvHceZQ41OR7ez8ljyjt9dmDPWaY0cYFNLdw0_L0JV1J8",
										"width": "225",
										"height": "225"
									}
								],
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/b95a580c-0f76-4097-bc0c-4f1507a65bb8.jpg",
										"og:type": "website",
										"twitter:card": "summary_large_image",
										"twitter:title": "Vertere Chart by Mind Against: Tracks on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Vertere Chart by Mind Against: Tracks on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Vertere Chart by Mind Against: Tracks on Beatport",
										"twitter:image": "https://geo-media.beatport.com/image/b95a580c-0f76-4097-bc0c-4f1507a65bb8.jpg",
										"twitter:image:alt": "Vertere Chart by Mind Against: Tracks on Beatport",
										"twitter:site": "@beatport",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:description": "Vertere Chart by Mind Against: Tracks on Beatport",
										"og:url": "https://www.beatport.com/chart/vertere-chart/356883"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/b95a580c-0f76-4097-bc0c-4f1507a65bb8.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Vertere from PHOBIA Music Recordings on Beatport",
							"htmlTitle": "\u003cb\u003eVertere\u003c/b\u003e from PHOBIA Music Recordings on Beatport",
							"link": "https://www.beatport.com/release/vertere/2773100",
							"displayLink": "www.beatport.com",
							"snippet": "Vertere. Georgi Z; Release Date 2019-12-06; Label PHOBIA Music Recordings; \nCatalog PMR035. $2.58. Title. Artists. Remixers. Genre. BPM. Key. Length. 1.",
							"htmlSnippet": "\u003cb\u003eVertere\u003c/b\u003e. Georgi Z; Release Date 2019-12-06; Label PHOBIA Music Recordings; \u003cbr\u003e\nCatalog PMR035. $2.58. Title. Artists. Remixers. Genre. BPM. Key. Length. 1.",
							"cacheId": "FKM6zeA_3uAJ",
							"formattedUrl": "https://www.beatport.com/release/vertere/2773100",
							"htmlFormattedUrl": "https://www.beatport.com/release/\u003cb\u003evertere\u003c/b\u003e/2773100",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"og:type": "website",
										"twitter:card": "summary_large_image",
										"twitter:title": "Vertere from PHOBIA Music Recordings on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Vertere from PHOBIA Music Recordings on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Bulgarian producer Georgi Z make his debut with two original tracks for PHOBIA . Grab your legal copy now.",
										"twitter:image": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"twitter:image:alt": "Vertere from PHOBIA Music Recordings on Beatport",
										"twitter:site": "@beatport",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:description": "Bulgarian producer Georgi Z make his debut with two original tracks for PHOBIA . Grab your legal copy now.",
										"og:url": "https://www.beatport.com/release/vertere/2773100"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/bfbf1528-34ff-4122-a462-415a1173fafd.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Georgi Z - Vertere (Original Mix) [PHOBIA Music Recordings ...",
							"htmlTitle": "Georgi Z - \u003cb\u003eVertere\u003c/b\u003e (Original Mix) [PHOBIA Music Recordings ...",
							"link": "http://classic.beatport.com/track/vertere-original-mix/12797583",
							"displayLink": "classic.beatport.com",
							"snippet": "Dec 6, 2019 ... Download Vertere (Original Mix) by Georgi Z from the album Vertere. Released \nby PHOBIA Music Recordings on Beatport, the world's largest ...",
							"htmlSnippet": "Dec 6, 2019 \u003cb\u003e...\u003c/b\u003e Download \u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Georgi Z from the album \u003cb\u003eVertere\u003c/b\u003e. Released \u003cbr\u003e\nby PHOBIA Music Recordings on Beatport, the world&#39;s largest&nbsp;...",
							"formattedUrl": "classic.beatport.com/track/vertere-original-mix/12797583",
							"htmlFormattedUrl": "classic.beatport.com/track/\u003cb\u003evertere\u003c/b\u003e-original-mix/12797583",
							"pagemap": {
								"cse_thumbnail": [
									{
										"src": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTfhgUaeWkmL8Q9-EBMH73PvsFysLNJ2YsD_dyVOKzKwzerJjY1BqbECCWF",
										"width": "138",
										"height": "138"
									}
								],
								"metatags": [
									{
										"og:image": "http://geo-media.beatport.com/image_size/500x500/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"copyright": "Copyright (c) 2020 Beatport, LLC. All rights reserved.",
										"og:type": "song",
										"og:site_name": "Beatport",
										"og:email": "support@beatport.com",
										"author": "Beatport, LLC",
										"og:title": "Georgi Z - Vertere (Original Mix) [PHOBIA Music Recordings]",
										"distribution": "global",
										"title": "Georgi Z - Vertere (Original Mix) [PHOBIA Music Recordings]",
										"og:video:secure_url": "https://static.bpddn.com/s/41c4e57d50588b50168f35cbc6d00f2b/swf/EmbeddablePlayer.swf?id=12797583&type=track&config_url=https%3A%2F%2Fembed.beatport.com%2Fplayer-config&auto=1&bg=0&nologo=0&noshare=0",
										"og:description": "Vertere (Original Mix) [PHOBIA Music Recordings] is available for download on Beatport, the world's largest DJ and electronic music community.",
										"og:video:width": "398",
										"og:image:secure_url": "https://geo-media.beatport.com/image_size/500x500/bfbf1528-34ff-4122-a462-415a1173fafd.jpg",
										"fb:app_id": "144706388881233",
										"og:video": "https://static.bpddn.com/s/41c4e57d50588b50168f35cbc6d00f2b/swf/EmbeddablePlayer.swf?id=12797583&type=track&config_url=https%3A%2F%2Fembed.beatport.com%2Fplayer-config&auto=1&bg=0&nologo=0&noshare=0",
										"name": "Beatport",
										"og:video:type": "application/x-shockwave-flash",
										"og:video:height": "162",
										"og:url": "http://classic.beatport.com/track/vertere-original-mix/12797583",
										"email": "support@beatport.com",
										"twitter:account_id": "31141686"
									}
								],
								"cse_image": [
									{
										"src": "http://geo-media.beatport.com/image_size/138x138/bfbf1528-34ff-4122-a462-415a1173fafd.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
							"htmlTitle": "Charts with \u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Mind Against, Somne on Beatport",
							"link": "https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?appid=Sushi&perPage=50&countryCode=US&realtimePrices=true&sourceType=sushi&format=json&page=4&id=6755315",
							"displayLink": "www.beatport.com",
							"snippet": "Download Now on Beatport.",
							"htmlSnippet": "Download Now on Beatport.",
							"cacheId": "-59Vftm1igMJ",
							"formattedUrl": "https://www.beatport.com/.../vertere.../charted-on?...50...",
							"htmlFormattedUrl": "https://www.beatport.com/.../\u003cb\u003evertere\u003c/b\u003e.../charted-on?...50...",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"og:type": "website",
										"twitter:card": "summary_large_image",
										"twitter:title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/3f27639acc59553ae15d1846ffe65cf2a63629ea/images/mstile-144x144.png",
										"og:description": "Download Now on Beatport.",
										"twitter:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"twitter:image:alt": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"twitter:site": "@beatport",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:description": "Download Now on Beatport.",
										"og:url": "https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?appid=Sushi&perPage=50&countryCode=US&realtimePrices=true&sourceType=sushi&format=json&page=4&id=6755315"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
							"htmlTitle": "Charts with \u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Mind Against, Somne on Beatport",
							"link": "https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?countryCode=US&perPage=50&id=6755315&page=2&appid=Sushi&sourceType=sushi&realtimePrices=true&format=json",
							"displayLink": "www.beatport.com",
							"snippet": "Vertere · 1 2 3 4. Jump to page. Results per page. 25 50 100 150. Humanoid \nChart. Lunar Plane. $13.41. Simon Doty Summer Favorites 2015. Simon Doty.",
							"htmlSnippet": "\u003cb\u003eVertere\u003c/b\u003e &middot; 1 2 3 4. Jump to page. Results per page. 25 50 100 150. Humanoid \u003cbr\u003e\nChart. Lunar Plane. $13.41. Simon Doty Summer Favorites 2015. Simon Doty.",
							"cacheId": "KHJpxyXUmRMJ",
							"formattedUrl": "https://www.beatport.com/.../vertere.../charted-on?...US...",
							"htmlFormattedUrl": "https://www.beatport.com/.../\u003cb\u003evertere\u003c/b\u003e.../charted-on?...US...",
							"pagemap": {
								"metatags": [
									{
										"msapplication-tilecolor": "#94d500",
										"application-name": "Beatport",
										"og:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"og:type": "website",
										"twitter:card": "summary_large_image",
										"twitter:title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"apple-mobile-web-app-title": "Beatport",
										"og:title": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"msapplication-tileimage": "https://geo-pro.beatport.com/static/58d30ba3c46f7594136d4b76d9965553.png",
										"og:description": "Download Now on Beatport.",
										"twitter:image": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"twitter:image:alt": "Charts with Vertere (Original Mix) by Mind Against, Somne on Beatport",
										"twitter:site": "@beatport",
										"viewport": "width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no, minimal-ui",
										"twitter:description": "Download Now on Beatport.",
										"og:url": "https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?countryCode=US&perPage=50&id=6755315&page=2&appid=Sushi&sourceType=sushi&realtimePrices=true&format=json"
									}
								],
								"cse_image": [
									{
										"src": "https://geo-media.beatport.com/image/4a3c628c-9022-429e-95ac-766abed62b09.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Mind Against, Somne - Vertere (Original Mix) [Life And Death ...",
							"htmlTitle": "Mind Against, Somne - \u003cb\u003eVertere\u003c/b\u003e (Original Mix) [Life And Death ...",
							"link": "http://classic.beatport.com/track/vertere-original-mix/6755315",
							"displayLink": "classic.beatport.com",
							"snippet": "Jun 29, 2015 ... Download Vertere (Original Mix) by Mind Against, Somne from the album Vertere. \nReleased by Life And Death on Beatport, the world's largest ...",
							"htmlSnippet": "Jun 29, 2015 \u003cb\u003e...\u003c/b\u003e Download \u003cb\u003eVertere\u003c/b\u003e (Original Mix) by Mind Against, Somne from the album \u003cb\u003eVertere\u003c/b\u003e. \u003cbr\u003e\nReleased by Life And Death on Beatport, the world&#39;s largest&nbsp;...",
							"formattedUrl": "classic.beatport.com/track/vertere-original-mix/6755315",
							"htmlFormattedUrl": "classic.beatport.com/track/\u003cb\u003evertere\u003c/b\u003e-original-mix/6755315",
							"pagemap": {
								"cse_thumbnail": [
									{
										"src": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQnL9erGND6tC_xRxspvWot6H9lNGxTcbzejeus9nVU6-G6QmJF0KF5yg",
										"width": "120",
										"height": "120"
									}
								],
								"metatags": [
									{
										"og:image": "http://geo-media.beatport.com/image_size/500x500/4a3c628c-9022-429e-95ac-766abe100 28587    0 28587    0     0  22160      0 --:--:--  0:00:01 --:--:-- 22160d62b09.jpg",
										"copyright": "Copyright (c) 2020 Beatport, LLC. All rights reserved.",
										"og:type": "song",
										"og:site_name": "Beatport",
										"og:email": "support@beatport.com",
										"author": "Beatport, LLC",
										"og:title": "Mind Against, Somne - Vertere (Original Mix) [Life And Death]",
										"distribution": "global",
										"title": "Mind Against, Somne - Vertere (Original Mix) [Life And Death]",
										"og:video:secure_url": "https://static.bpddn.com/s/8dc420b4e04e440e5285c101768d2208/swf/EmbeddablePlayer.swf?id=6755315&type=track&config_url=https%3A%2F%2Fembed.beatport.com%2Fplayer-config&auto=1&bg=0&nologo=0&noshare=0",
										"og:description": "Vertere (Original Mix) [Life And Death] is available for download on Beatport, the world's largest DJ and electronic music community.",
										"og:video:width": "398",
										"og:image:secure_url": "https://geo-media.beatport.com/image_size/500x500/4a3c628c-9022-429e-95ac-766abed62b09.jpg",
										"fb:app_id": "144706388881233",
										"og:video": "https://static.bpddn.com/s/8dc420b4e04e440e5285c101768d2208/swf/EmbeddablePlayer.swf?id=6755315&type=track&config_url=https%3A%2F%2Fembed.beatport.com%2Fplayer-config&auto=1&bg=0&nologo=0&noshare=0",
										"name": "Beatport",
										"og:video:type": "application/x-shockwave-flash",
										"og:video:height": "162",
										"og:url": "http://classic.beatport.com/track/vertere-original-mix/6755315",
										"email": "support@beatport.com",
										"twitter:account_id": "31141686"
									}
								],
								"cse_image": [
									{
										"src": "http://geo-media.beatport.com/image_size/120x120/68cfc1f0-acaa-4554-aba8-ef8050b84283.jpg"
									}
								]
							}
						},
						{
							"kind": "customsearch#result",
							"title": "Confusion [PHOBIA Music Recordings] :: Beatport",
							"htmlTitle": "Confusion [PHOBIA Music Recordings] :: Beatport",
							"link": "http://classic.beatport.com/release/confusion/2795174",
							"displayLink": "classic.beatport.com",
							"snippet": "Jan 10, 2020 ... play queue. Rock Is Back · Ataman Live, Ataman Livе, Vadoz · PHOBIA Music \nRecordings | 2019-05-20. Georgi Z - Vertere. play queue. Vertere",
							"htmlSnippet": "Jan 10, 2020 \u003cb\u003e...\u003c/b\u003e play queue. Rock Is Back &middot; Ataman Live, Ataman Livе, Vadoz &middot; PHOBIA Music \u003cbr\u003e\nRecordings | 2019-05-20. Georgi Z - \u003cb\u003eVertere\u003c/b\u003e. play queue. \u003cb\u003eVertere\u003c/b\u003e",
							"formattedUrl": "classic.beatport.com/release/confusion/2795174",
							"htmlFormattedUrl": "classic.beatport.com/release/confusion/2795174",
							"pagemap": {
								"metatags": [
									{
										"og:image": "http://geo-media.beatport.com/image_size/500x500/3b641cb0-64fe-44c5-878f-41ae0b629f6a.jpg",
										"copyright": "Copyright (c) 2020 Beatport, LLC. All rights reserved.",
										"og:type": "album",
										"og:site_name": "Beatport",
										"og:email": "support@beatport.com",
										"author": "Beatport, LLC",
										"og:title": "Confusion [PHOBIA Music Recordings]",
										"distribution": "global",
										"title": "Confusion [PHOBIA Music Recordings]",
										"og:description": "Anthon and MadChords from Lebanon are our new artists . Grab your legal copy now !",
										"og:image:secure_url": "https://geo-media.beatport.com/image_size/500x500/3b641cb0-64fe-44c5-878f-41ae0b629f6a.jpg",
										"fb:app_id": "144706388881233",
										"name": "Beatport",
										"og:url": "http://classic.beatport.com/release/confusion/2795174",
										"email": "support@beatport.com",
										"twitter:account_id": "31141686"
									}
								],
								"cse_image": [
									{
										"src": "http://geo-media.beatport.com/image_size/500x500/3b641cb0-64fe-44c5-878f-41ae0b629f6a.jpg"
									}
								]
							}
						}
					]
				}
			"####
		)
		.expect("invalid test payload");

	assert_eq!(
		payload,
		Items(
			Box::new([
				"https://www.beatport.com/track/vertere-original-mix/6755315".into(),
				"https://www.beatport.com/release/vertere/1549121".into(),
				"https://www.beatport.com/track/vertere-original-mix/12797583".into(),
				"https://www.beatport.com/chart/vertere-chart/356883".into(),
				"https://www.beatport.com/release/vertere/2773100".into(),
				"http://classic.beatport.com/track/vertere-original-mix/12797583".into(),
				"https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?appid=Sushi&perPage=50&countryCode=US&realtimePrices=true&sourceType=sushi&format=json&page=4&id=6755315".into(),
				"https://www.beatport.com/track/vertere-original-mix/6755315/charted-on?countryCode=US&perPage=50&id=6755315&page=2&appid=Sushi&sourceType=sushi&realtimePrices=true&format=json".into(),
				"http://classic.beatport.com/track/vertere-original-mix/6755315".into(),
				"http://classic.beatport.com/release/confusion/2795174".into(),
			])
		),
	);
}
