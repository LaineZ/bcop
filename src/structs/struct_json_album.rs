#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current: Current,
    #[serde(rename = "album_release_date")]
    pub album_release_date: Option<String>,
    #[serde(rename = "preorder_count")]
    pub preorder_count: ::serde_json::Value,
    pub has_audio: Option<bool>,
    #[serde(rename = "art_id")]
    pub art_id: Option<i64>,
    pub trackinfo: Option<Vec<Trackinfo>>,
    #[serde(rename = "playing_from")]
    pub playing_from: Option<String>,
    #[serde(rename = "featured_track_id")]
    pub featured_track_id: Option<i64>,
    #[serde(rename = "initial_track_num")]
    pub initial_track_num: ::serde_json::Value,
    pub packages: Option<Vec<Package>>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(rename = "new_date")]
    pub new_date: Option<String>,
    #[serde(rename = "featured_track_id")]
    pub featured_track_id: Option<i64>,
    pub upc: ::serde_json::Value,
    #[serde(rename = "purchase_url")]
    pub purchase_url: ::serde_json::Value,
    #[serde(rename = "download_desc_id")]
    pub download_desc_id: ::serde_json::Value,
    #[serde(rename = "new_desc_format")]
    pub new_desc_format: Option<i64>,
    pub artist: Option<String>,
    #[serde(rename = "auto_repriced")]
    pub auto_repriced: ::serde_json::Value,
    #[serde(rename = "set_price")]
    pub set_price: Option<f64>,
    #[serde(rename = "purchase_title")]
    pub purchase_title: ::serde_json::Value,
    #[serde(rename = "selling_band_id")]
    pub selling_band_id: Option<i64>,
    #[serde(rename = "minimum_price_nonzero")]
    pub minimum_price_nonzero: Option<f64>,
    #[serde(rename = "download_pref")]
    pub download_pref: Option<i64>,
    pub audit: Option<i64>,
    pub private: ::serde_json::Value,
    pub about: Option<String>,
    pub title: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "art_id")]
    pub art_id: Option<i64>,
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<f64>,
    #[serde(rename = "mod_date")]
    pub mod_date: Option<String>,
    #[serde(rename = "band_id")]
    pub band_id: Option<i64>,
    pub credits: ::serde_json::Value,
    #[serde(rename = "is_set_price")]
    pub is_set_price: ::serde_json::Value,
    #[serde(rename = "require_email")]
    pub require_email: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "release_date")]
    pub release_date: Option<String>,
    #[serde(rename = "require_email_0")]
    pub require_email0: ::serde_json::Value,
    pub killed: ::serde_json::Value,
    #[serde(rename = "publish_date")]
    pub publish_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trackinfo {
    #[serde(rename = "video_featured")]
    pub video_featured: ::serde_json::Value,
    #[serde(rename = "has_lyrics")]
    pub has_lyrics: Option<bool>,
    #[serde(rename = "video_source_type")]
    pub video_source_type: ::serde_json::Value,
    pub streaming: Option<i64>,
    #[serde(rename = "alt_link")]
    pub alt_link: ::serde_json::Value,
    #[serde(rename = "has_info")]
    pub has_info: Option<bool>,
    #[serde(rename = "video_id")]
    pub video_id: ::serde_json::Value,
    #[serde(rename = "video_source_id")]
    pub video_source_id: ::serde_json::Value,
    #[serde(rename = "track_num")]
    pub track_num: Option<i64>,
    #[serde(rename = "encoding_error")]
    pub encoding_error: ::serde_json::Value,
    pub lyrics: ::serde_json::Value,
    pub duration: Option<f64>,
    #[serde(rename = "is_downloadable")]
    pub is_downloadable: Option<bool>,
    #[serde(rename = "license_type")]
    pub license_type: Option<i64>,
    #[serde(rename = "video_mobile_url")]
    pub video_mobile_url: ::serde_json::Value,
    #[serde(rename = "album_preorder")]
    pub album_preorder: Option<bool>,
    pub private: ::serde_json::Value,
    #[serde(rename = "encoding_pending")]
    pub encoding_pending: ::serde_json::Value,
    #[serde(rename = "is_draft")]
    pub is_draft: Option<bool>,
    #[serde(rename = "has_free_download")]
    pub has_free_download: ::serde_json::Value,
    pub title: Option<String>,
    #[serde(rename = "track_license_id")]
    pub track_license_id: ::serde_json::Value,
    #[serde(rename = "video_poster_url")]
    pub video_poster_url: ::serde_json::Value,
    #[serde(rename = "unreleased_track")]
    pub unreleased_track: Option<bool>,
    pub id: Option<i64>,
    #[serde(rename = "play_count")]
    pub play_count: Option<i64>,
    #[serde(rename = "free_album_download")]
    pub free_album_download: Option<bool>,
    #[serde(rename = "track_id")]
    pub track_id: Option<i64>,
    #[serde(rename = "video_caption")]
    pub video_caption: ::serde_json::Value,
    #[serde(rename = "title_link")]
    pub title_link: Option<String>,
    #[serde(rename = "encodings_id")]
    pub encodings_id: Option<i64>,
    pub file: File,
    #[serde(rename = "is_capped")]
    pub is_capped: Option<bool>,
    #[serde(rename = "sizeof_lyrics")]
    pub sizeof_lyrics: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(rename = "mp3-128")]
    pub mp3128: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "album_private")]
    pub album_private: ::serde_json::Value,
    #[serde(rename = "quantity_warning")]
    pub quantity_warning: Option<bool>,
    #[serde(rename = "new_date")]
    pub new_date: Option<String>,
    pub origins: Option<Vec<Origin>>,
    #[serde(rename = "certified_seller")]
    pub certified_seller: Option<i64>,
    #[serde(rename = "featured_date")]
    pub featured_date: ::serde_json::Value,
    #[serde(rename = "quantity_available")]
    pub quantity_available: Option<i64>,
    #[serde(rename = "download_has_audio")]
    pub download_has_audio: Option<bool>,
    pub currency: Option<String>,
    pub upc: ::serde_json::Value,
    #[serde(rename = "subscriber_only_published")]
    pub subscriber_only_published: Option<bool>,
    #[serde(rename = "album_id")]
    pub album_id: Option<i64>,
    pub label: Option<String>,
    #[serde(rename = "new_desc_format")]
    pub new_desc_format: Option<i64>,
    #[serde(rename = "quantity_sold")]
    pub quantity_sold: ::serde_json::Value,
    #[serde(rename = "download_art_id")]
    pub download_art_id: Option<i64>,
    #[serde(rename = "album_art")]
    pub album_art: ::serde_json::Value,
    pub options: ::serde_json::Value,
    #[serde(rename = "selling_band_id")]
    pub selling_band_id: Option<i64>,
    #[serde(rename = "desc_pt1")]
    pub desc_pt1: Option<String>,
    #[serde(rename = "associated_license_id")]
    pub associated_license_id: ::serde_json::Value,
    #[serde(rename = "album_publish_date")]
    pub album_publish_date: Option<String>,
    #[serde(rename = "album_title")]
    pub album_title: Option<String>,
    #[serde(rename = "fulfillment_days")]
    pub fulfillment_days: Option<i64>,
    #[serde(rename = "download_artist")]
    pub download_artist: Option<String>,
    #[serde(rename = "shipping_exception_mode")]
    pub shipping_exception_mode: ::serde_json::Value,
    #[serde(rename = "options_title")]
    pub options_title: ::serde_json::Value,
    pub private: ::serde_json::Value,
    #[serde(rename = "album_release_date")]
    pub album_release_date: Option<String>,
    #[serde(rename = "download_type")]
    pub download_type: Option<String>,
    #[serde(rename = "album_art_id")]
    pub album_art_id: Option<i64>,
    pub arts: Option<Vec<Art>>,
    #[serde(rename = "desc_pt2")]
    pub desc_pt2: ::serde_json::Value,
    pub description: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "edition_size")]
    pub edition_size: ::serde_json::Value,
    #[serde(rename = "download_id")]
    pub download_id: Option<i64>,
    #[serde(rename = "tax_rate")]
    pub tax_rate: ::serde_json::Value,
    pub country: ::serde_json::Value,
    #[serde(rename = "type_name")]
    pub type_name: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "album_artist")]
    pub album_artist: Option<String>,
    #[serde(rename = "download_url")]
    pub download_url: Option<String>,
    #[serde(rename = "band_id")]
    pub band_id: Option<i64>,
    #[serde(rename = "is_set_price")]
    pub is_set_price: ::serde_json::Value,
    #[serde(rename = "subscriber_only")]
    pub subscriber_only: ::serde_json::Value,
    #[serde(rename = "grid_index")]
    pub grid_index: Option<i64>,
    #[serde(rename = "type_id")]
    pub type_id: Option<i64>,
    #[serde(rename = "quantity_limits")]
    pub quantity_limits: Option<i64>,
    #[serde(rename = "release_date")]
    pub release_date: ::serde_json::Value,
    pub sku: Option<String>,
    pub price: Option<f64>,
    #[serde(rename = "download_title")]
    pub download_title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    #[serde(rename = "package_id")]
    pub package_id: Option<i64>,
    #[serde(rename = "quantity_available")]
    pub quantity_available: Option<i64>,
    #[serde(rename = "quantity_sold")]
    pub quantity_sold: ::serde_json::Value,
    pub quantity: ::serde_json::Value,
    #[serde(rename = "option_id")]
    pub option_id: Option<i64>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Art {
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(rename = "image_id")]
    pub image_id: Option<i64>,
    pub index: Option<i64>,
    pub id: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCapData {
    #[serde(rename = "streaming_limits_enabled")]
    pub streaming_limits_enabled: Option<bool>,
    #[serde(rename = "streaming_limit")]
    pub streaming_limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TralbumCollectInfo {
    #[serde(rename = "show_wishlist_tooltip")]
    pub show_wishlist_tooltip: Option<bool>,
    #[serde(rename = "show_collect")]
    pub show_collect: Option<bool>,
}
