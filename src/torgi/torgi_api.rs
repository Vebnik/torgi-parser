use crate::torgi::TorgiResponse;

use super::torgi_types::Lot;

impl Lot {
    pub async fn search(query: String, pages: i32) -> Result<Vec<Lot>, ()> {
        let url = "https://torgi.gov.ru/new/api/public/lotcards/search";

        let mut results: Vec<Lot> = Vec::with_capacity(1000);

        for page in 0..pages {
            let querystring = [
                ("text", query.clone()),
                ("matchPhrase", "false".to_string()),
                ("byFirstVersion", "true".to_string()),
                ("withFacets", "false".to_string()),
                ("page", page.to_string()),
                ("size", "10".to_string()),
                ("sort", "firstVersionPublicationDate,desc".to_string()),
            ];

            let client = reqwest::Client::new();
            let response = client.get(url).query(&querystring).send().await;

            match response {
                Ok(response) => {
                    let result = response.json::<TorgiResponse>().await;

                    match result {
                        Ok(result) => {
                            results.append(&mut result.content.clone());
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

    #[allow(dead_code)]
    pub async fn get_by_id(id: String) -> Result<Lot, ()> {
        let url = format!("https://torgi.gov.ru/new/api/public/lotcards/{id}");

        let client = reqwest::Client::new();
        let response = client.get(url).send().await;

        let results = response.unwrap().json::<Lot>().await.unwrap();

        Ok(results)
    }
}
