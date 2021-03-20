use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub genre: Genre,
    pub tag: Tag,
    pub auto: Auto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genre {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub count: i64,
    pub matches: Vec<Match>,
    pub time_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub norm_name: String,
    pub display_tag_id: i64,
    pub count: i64,
    pub score: i64,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auto {
    pub results: Vec<Result>,
    pub stat_params_for_tag: String,
    pub time_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub art_id: Option<i64>,
    pub url: String,
    pub is_label: Option<bool>,
    pub img_id: Option<i64>,
    pub id: Option<i64>,
    pub stat_params: Option<String>,
    #[serde(rename = "type")]
    pub field_type: String,
    pub part: Option<String>,
    pub img: Option<String>,
    pub name: Option<String>,
    pub band_name: Option<String>,
    pub band_id: Option<i64>,
}
