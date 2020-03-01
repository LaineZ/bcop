use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub current: Current,
    pub album_release_date: Option<String>,
    pub preorder_count: ::serde_json::Value,
    pub has_audio: Option<bool>,
    pub art_id: Option<i64>,
    pub trackinfo: Option<Vec<Trackinfo>>,
    pub playing_from: Option<String>,
    pub featured_track_id: Option<i64>,
    pub initial_track_num: ::serde_json::Value,
    pub packages: Option<Vec<Package>>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Current {
    pub new_date: Option<String>,
    pub featured_track_id: Option<i64>,
    pub upc: ::serde_json::Value,
    pub purchase_url: ::serde_json::Value,
    pub download_desc_id: ::serde_json::Value,
    pub new_desc_format: Option<i64>,
    pub artist: Option<String>,
    pub auto_repriced: ::serde_json::Value,
    pub set_price: Option<f64>,
    pub purchase_title: ::serde_json::Value,
    pub selling_band_id: Option<i64>,
    pub minimum_price_nonzero: Option<f64>,
    pub download_pref: Option<i64>,
    pub audit: Option<i64>,
    pub private: ::serde_json::Value,
    pub about: Option<String>,
    pub title: Option<String>,
    pub id: Option<i64>,
    pub art_id: Option<i64>,
    pub minimum_price: Option<f64>,
    pub mod_date: Option<String>,
    pub band_id: Option<i64>,
    pub credits: ::serde_json::Value,
    pub is_set_price: ::serde_json::Value,
    pub require_email: ::serde_json::Value,
    pub type_field: Option<String>,
    pub release_date: Option<String>,
    pub require_email_0: ::serde_json::Value,
    pub killed: ::serde_json::Value,
    pub publish_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trackinfo {
    pub video_featured: ::serde_json::Value,
    pub has_lyrics: Option<bool>,
    pub video_source_type: ::serde_json::Value,
    pub streaming: Option<i64>,
    pub alt_link: ::serde_json::Value,
    pub has_info: Option<bool>,
    pub video_id: ::serde_json::Value,
    pub video_source_id: ::serde_json::Value,
    pub track_num: Option<i64>,
    pub encoding_error: ::serde_json::Value,
    pub lyrics: ::serde_json::Value,
    pub duration: Option<f64>,
    pub is_downloadable: Option<bool>,
    pub license_type: Option<i64>,
    pub video_mobile_url: ::serde_json::Value,
    pub album_preorder: Option<bool>,
    pub private: ::serde_json::Value,
    pub encoding_pending: ::serde_json::Value,
    pub is_draft: Option<bool>,
    pub has_free_download: ::serde_json::Value,
    pub title: Option<String>,
    pub track_license_id: ::serde_json::Value,
    pub video_poster_url: ::serde_json::Value,
    pub unreleased_track: Option<bool>,
    pub id: Option<i64>,
    pub play_count: Option<i64>,
    pub free_album_download: Option<bool>,
    pub track_id: Option<i64>,
    pub video_caption: ::serde_json::Value,
    pub title_link: Option<String>,
    pub encodings_id: Option<i64>,
    pub file: File,
    pub is_capped: Option<bool>,
    pub sizeof_lyrics: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "mp3-128")]
    pub mp3128: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub album_private: ::serde_json::Value,
    pub quantity_warning: Option<bool>,
    pub new_date: Option<String>,
    pub origins: Option<Vec<Origin>>,
    pub certified_seller: Option<i64>,
    pub featured_date: ::serde_json::Value,
    pub quantity_available: Option<i64>,
    pub download_has_audio: Option<bool>,
    pub currency: Option<String>,
    pub upc: ::serde_json::Value,
    pub subscriber_only_published: Option<bool>,
    pub album_id: Option<i64>,
    pub label: Option<String>,
    pub new_desc_format: Option<i64>,
    pub quantity_sold: ::serde_json::Value,
    pub download_art_id: Option<i64>,
    pub album_art: ::serde_json::Value,
    pub options: ::serde_json::Value,
    pub selling_band_id: Option<i64>,
    pub desc_pt1: Option<String>,
    pub associated_license_id: ::serde_json::Value,
    pub album_publish_date: Option<String>,
    pub album_title: Option<String>,
    pub fulfillment_days: Option<i64>,
    pub download_artist: Option<String>,
    pub shipping_exception_mode: ::serde_json::Value,
    pub options_title: ::serde_json::Value,
    pub private: ::serde_json::Value,
    pub album_release_date: Option<String>,
    pub download_type: Option<String>,
    pub album_art_id: Option<i64>,
    pub arts: Option<Vec<Art>>,
    pub desc_pt2: ::serde_json::Value,
    pub description: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub edition_size: ::serde_json::Value,
    pub download_id: Option<i64>,
    pub tax_rate: ::serde_json::Value,
    pub country: ::serde_json::Value,
    pub type_name: Option<String>,
    pub id: Option<i64>,
    pub album_artist: Option<String>,
    pub download_url: Option<String>,
    pub band_id: Option<i64>,
    pub is_set_price: ::serde_json::Value,
    pub subscriber_only: ::serde_json::Value,
    pub grid_index: Option<i64>,
    pub type_id: Option<i64>,
    pub quantity_limits: Option<i64>,
    pub release_date: ::serde_json::Value,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub download_title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Origin {
    pub package_id: Option<i64>,
    pub quantity_available: Option<i64>,
    pub quantity_sold: ::serde_json::Value,
    pub quantity: ::serde_json::Value,
    pub option_id: Option<i64>,
    pub id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Art {
    pub file_name: Option<String>,
    pub image_id: Option<i64>,
    pub index: Option<i64>,
    pub id: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayCapData {
    pub streaming_limits_enabled: Option<bool>,
    pub streaming_limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TralbumCollectInfo {
    pub show_wishlist_tooltip: Option<bool>,
    pub show_collect: Option<bool>,
}
