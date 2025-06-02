use serde::{self, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lot {
    pub full_url: Option<String>,
    pub title: String,
    pub description: String,
    pub id: i32,
    pub added: String,
    pub total_area: String,
    pub category: String,
    pub formatted_full_price: String,
    pub formatted_full_info: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CianDataResponse {
    pub offers_serialized: Vec<Lot>,
    pub offer_count: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CianResponse {
    pub data: CianDataResponse,
    pub ok: String,
}
