use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lot {
    pub id: String,
    pub lot_name: Value,
    pub price_min: Value,
    pub bidd_form: BiddForm,
    pub lot_number: Value,
    pub bidd_type: BiddType,
    pub lot_description: Value,
    pub lot_images: Vec<Value>,
    pub category: Category,
    pub notice_first_version_publication_date: Value,
    pub is_annulled: bool,
    pub characteristics: Option<Vec<Characteristic>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiddForm {
    pub code: Value,
    pub name: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiddType {
    pub code: Value,
    pub name: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub code: Value,
    pub name: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorgiResponse {
    pub content: Vec<Lot>,
    pub empty: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub characteristic_value: Option<Value>,
    pub name: Value,
    pub code: Value,
    #[serde(rename = "type")]
    pub type_field: Value,
    pub unit: Option<Value>,
}