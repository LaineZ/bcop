#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub discover_spec: DiscoverSpec,
    pub more_available: bool,
    pub ok: bool,
    pub filters: String,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DiscoverSpec {
    pub format_type_id: i64,
    pub tag_pretty_name: String,
    pub dig_deeper_followed: bool,
    pub format: String,
    pub geoname_name: ::serde_json::Value,
    pub genre_id: i64,
    pub discover_id: i64,
    pub genre_pretty_name: ::serde_json::Value,
    pub genre_name: ::serde_json::Value,
    pub spec_name: String,
    pub tag_name: String,
    pub geoname_id: i64,
    pub tag_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item {
    pub band_url: String,
    pub featured_track_title: String,
    pub artist: String,
    pub packages: Vec<Package>,
    pub audio_url: AudioUrl,
    pub featured_track_number: i64,
    pub custom_domain: Option<String>,
    pub genre_id: Option<i64>,
    pub audio_track_id: i64,
    pub tralbum_url: String,
    pub tralbum_type: String,
    pub title: String,
    pub subdomain: String,
    pub custom_domain_verified: Option<i64>,
    pub art_id: i64,
    pub num_comments: i64,
    pub genre: String,
    pub tralbum_id: i64,
    pub band_name: String,
    pub slug_text: String,
    pub item_type: String,
    pub band_id: i64,
    pub is_preorder: Option<i64>,
    pub item_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Package {
    pub currency: String,
    pub quantity: ::serde_json::Value,
    pub type_str: String,
    pub image_ids: Vec<i64>,
    pub is_vinyl: bool,
    pub id: i64,
    pub image: Image,
    pub is_set_price: Option<i64>,
    pub price: Price,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image {
    pub id: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Price {
    pub currency: String,
    pub is_money: bool,
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AudioUrl {
    #[serde(rename = "mp3-128")]
    pub mp3128: String,
}
