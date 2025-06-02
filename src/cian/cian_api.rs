use serde_json::{Value, json};

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
        let mut object_type_key = "room";

        let object_type_value = match _type.as_str() {
            "flatsale" | "flatrent" => {
                object_type_key = "room";
                json!({ "type": "terms", "value": [ object ] })
            }

            "suburbansale" | "suburbanrent" => {
                object_type_key = "object_type";
                json!({ "type": "terms", "value": [ object ] })
            }

            "commercialsale" | "commercialrent" => {
                object_type_key = "office_type";
                json!({ "type": "terms", "value": [ object ] })
            }

            _ => json!({ "type": "terms", "value": [ object ] }),
        };

        for page in 1..=pages {
            let body: Value = json!({
                "jsonQuery": {
                    "_type": _type,
                    "region": {
                        "type": "terms",
                        "value": [ region ]
                    },
                    "engine_version": {
                        "type": "term",
                        "value": 2
                    },
                    "page": {
                        "type": "term",
                        "value": page
                    },
                    object_type_key: object_type_value
                }
            });

            log::info!("{}", body);

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
