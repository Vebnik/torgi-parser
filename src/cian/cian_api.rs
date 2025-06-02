use serde_json::json;

use super::{CianResponse, cian_types::Lot};

impl Lot {
    pub async fn search(
        _type: String,
        pages: i32,
        region: i32,
        object: i32,
    ) -> Result<Vec<Lot>, ()> {
        let url = "https://api.cian.ru/search-offers/v2/search-offers-desktop/";

        let mut results: Vec<Lot> = Vec::with_capacity(1000);

        for page in 0..pages {
            let body = json!({
                "jsonQuery": {
                    "region": {
                        "type": "terms",
                        "value": [ region ]
                    },
                    "_type": _type,
                    "engine_version": {
                        "type": "term",
                        "value": 2
                    },
                    "page": {
                        "type": "term",
                        "value": page
                    },
                    "room": {
                        "type": "terms",
                        "value": [ object ]
                    }
                }
            });

            let client = reqwest::Client::new();
            let response = client.post(url).json(&body).send().await;

            match response {
                Ok(response) => {
                    let result = response.json::<CianResponse>().await;

                    match result {
                        Ok(result) => {
                            results.append(&mut result.data.offers_serialized.clone());
                        }
                        Err(err) => {
                            log::error!("{err:#?}");
                        }
                    };
                }
                Err(err) => {
                    log::error!("{err:#?}");
                }
            }
        }

        Ok(results)
    }
}
