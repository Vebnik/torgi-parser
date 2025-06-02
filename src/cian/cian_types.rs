use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lot {
    pub full_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub id: i32,
    pub added: Option<String>,
    pub total_area: Option<String>,
    pub category: Option<String>,
    pub formatted_full_price: Option<String>,
    pub formatted_full_info: Option<String>,
    pub geo: Value,
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
    pub status: String,
}
