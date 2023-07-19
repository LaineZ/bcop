use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub ok: bool,
    pub filters: String,
    pub items: Vec<Item>,
    pub more_available: bool,
    pub discover_spec: DiscoverSpec,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub tralbum_type: String,
    pub tralbum_id: i64,
    pub item_id: i64,
    pub title: String,
    pub art_id: i64,
    pub audio_track_id: i64,
    pub featured_track_title: String,
    pub featured_track_number: i64,
    pub artist: String,
    pub band_name: String,
    pub subdomain: String,
    pub custom_domain: Value,
    pub custom_domain_verified: Value,
    pub genre_id: i64,
    pub slug_text: String,
    pub item_type: String,
    pub band_id: i64,
    pub is_preorder: Option<i64>,
    pub packages: Vec<Package>,
    pub num_comments: i64,
    pub band_url: String,
    pub tralbum_url: String,
    pub genre: String,
    pub audio_url: AudioUrl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub id: i64,
    pub price: Price,
    pub is_set_price: Value,
    pub currency: String,
    pub type_str: String,
    pub is_vinyl: bool,
    pub image_ids: Vec<i64>,
    pub image: Image,
    pub quantity: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub amount: i64,
    pub currency: String,
    pub is_money: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub id: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioUrl {
    #[serde(rename = "mp3-128")]
    pub mp3_128: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoverSpec {
    pub genre_id: i64,
    pub genre_name: Value,
    pub genre_pretty_name: Value,
    pub tag_id: i64,
    pub tag_name: String,
    pub tag_pretty_name: String,
    pub geoname_id: i64,
    pub geoname_name: Value,
    pub format_type_id: i64,
    pub format: String,
    pub spec_name: String,
    pub discover_id: i64,
    pub dig_deeper_followed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoverRequest {
    pub filters: Filters,
    pub page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filters {
    pub format: String,
    pub location: i64,
    pub sort: String,
    pub tags: Vec<String>,
}
