#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current: Current,
    #[serde(rename = "is_preorder")]
    pub is_preorder: ::serde_json::Value,
    #[serde(rename = "album_is_preorder")]
    pub album_is_preorder: ::serde_json::Value,
    #[serde(rename = "album_release_date")]
    pub album_release_date: String,
    #[serde(rename = "preorder_count")]
    pub preorder_count: ::serde_json::Value,
    pub has_audio: bool,
    #[serde(rename = "art_id")]
    pub art_id: i64,
    pub trackinfo: Vec<Trackinfo>,
    #[serde(rename = "playing_from")]
    pub playing_from: String,
    #[serde(rename = "featured_track_id")]
    pub featured_track_id: i64,
    #[serde(rename = "initial_track_num")]
    pub initial_track_num: ::serde_json::Value,
    pub packages: Vec<Package>,
    pub url: String,
    pub default_price: f64,
    pub free_download_page: ::serde_json::Value,
    #[serde(rename = "FREE")]
    pub free: i64,
    #[serde(rename = "PAID")]
    pub paid: i64,
    pub artist: String,
    #[serde(rename = "item_type")]
    pub item_type: String,
    pub id: i64,
    #[serde(rename = "last_subscription_item")]
    pub last_subscription_item: ::serde_json::Value,
    #[serde(rename = "has_discounts")]
    pub has_discounts: bool,
    #[serde(rename = "is_bonus")]
    pub is_bonus: ::serde_json::Value,
    #[serde(rename = "play_cap_data")]
    pub play_cap_data: PlayCapData,
    #[serde(rename = "client_id_sig")]
    pub client_id_sig: ::serde_json::Value,
    #[serde(rename = "is_purchased")]
    pub is_purchased: ::serde_json::Value,
    #[serde(rename = "items_purchased")]
    pub items_purchased: ::serde_json::Value,
    #[serde(rename = "is_private_stream")]
    pub is_private_stream: ::serde_json::Value,
    #[serde(rename = "is_band_member")]
    pub is_band_member: ::serde_json::Value,
    #[serde(rename = "licensed_version_ids")]
    pub licensed_version_ids: ::serde_json::Value,
    #[serde(rename = "package_associated_license_id")]
    pub package_associated_license_id: ::serde_json::Value,
    #[serde(rename = "tralbum_collect_info")]
    pub tralbum_collect_info: TralbumCollectInfo,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(rename = "is_set_price")]
    pub is_set_price: ::serde_json::Value,
    #[serde(rename = "require_email")]
    pub require_email: ::serde_json::Value,
    pub about: String,
    pub artist: ::serde_json::Value,
    #[serde(rename = "require_email_0")]
    pub require_email0: ::serde_json::Value,
    #[serde(rename = "minimum_price")]
    pub minimum_price: f64,
    #[serde(rename = "publish_date")]
    pub publish_date: String,
    #[serde(rename = "mod_date")]
    pub mod_date: String,
    #[serde(rename = "selling_band_id")]
    pub selling_band_id: i64,
    pub credits: ::serde_json::Value,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "auto_repriced")]
    pub auto_repriced: ::serde_json::Value,
    #[serde(rename = "purchase_url")]
    pub purchase_url: ::serde_json::Value,
    #[serde(rename = "download_desc_id")]
    pub download_desc_id: ::serde_json::Value,
    #[serde(rename = "new_desc_format")]
    pub new_desc_format: i64,
    pub killed: ::serde_json::Value,
    pub private: ::serde_json::Value,
    #[serde(rename = "art_id")]
    pub art_id: i64,
    #[serde(rename = "set_price")]
    pub set_price: f64,
    #[serde(rename = "new_date")]
    pub new_date: String,
    pub audit: i64,
    pub id: i64,
    #[serde(rename = "featured_track_id")]
    pub featured_track_id: i64,
    #[serde(rename = "purchase_title")]
    pub purchase_title: ::serde_json::Value,
    pub upc: String,
    #[serde(rename = "band_id")]
    pub band_id: i64,
    #[serde(rename = "minimum_price_nonzero")]
    pub minimum_price_nonzero: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "download_pref")]
    pub download_pref: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trackinfo {
    #[serde(rename = "encoding_error")]
    pub encoding_error: ::serde_json::Value,
    #[serde(rename = "is_downloadable")]
    pub is_downloadable: bool,
    pub streaming: i64,
    #[serde(rename = "license_type")]
    pub license_type: i64,
    #[serde(rename = "video_mobile_url")]
    pub video_mobile_url: ::serde_json::Value,
    #[serde(rename = "album_preorder")]
    pub album_preorder: bool,
    #[serde(rename = "encoding_pending")]
    pub encoding_pending: ::serde_json::Value,
    #[serde(rename = "is_draft")]
    pub is_draft: bool,
    #[serde(rename = "has_free_download")]
    pub has_free_download: ::serde_json::Value,
    #[serde(rename = "video_poster_url")]
    pub video_poster_url: ::serde_json::Value,
    pub duration: f64,
    #[serde(rename = "unreleased_track")]
    pub unreleased_track: bool,
    #[serde(rename = "play_count")]
    pub play_count: i64,
    #[serde(rename = "free_album_download")]
    pub free_album_download: bool,
    #[serde(rename = "video_caption")]
    pub video_caption: ::serde_json::Value,
    #[serde(rename = "title_link")]
    pub title_link: String,
    pub private: ::serde_json::Value,
    #[serde(rename = "is_capped")]
    pub is_capped: bool,
    #[serde(rename = "video_id")]
    pub video_id: ::serde_json::Value,
    #[serde(rename = "sizeof_lyrics")]
    pub sizeof_lyrics: i64,
    #[serde(rename = "video_featured")]
    pub video_featured: ::serde_json::Value,
    #[serde(rename = "has_lyrics")]
    pub has_lyrics: bool,
    #[serde(rename = "track_id")]
    pub track_id: i64,
    pub id: i64,
    #[serde(rename = "video_source_type")]
    pub video_source_type: ::serde_json::Value,
    #[serde(rename = "encodings_id")]
    pub encodings_id: i64,
    #[serde(rename = "alt_link")]
    pub alt_link: ::serde_json::Value,
    #[serde(rename = "has_info")]
    pub has_info: bool,
    pub file: File,
    #[serde(rename = "track_license_id")]
    pub track_license_id: ::serde_json::Value,
    #[serde(rename = "video_source_id")]
    pub video_source_id: ::serde_json::Value,
    pub lyrics: ::serde_json::Value,
    #[serde(rename = "track_num")]
    pub track_num: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(rename = "mp3-128")]
    pub mp3128: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "quantity_warning")]
    pub quantity_warning: bool,
    #[serde(rename = "quantity_available")]
    pub quantity_available: i64,
    pub country: ::serde_json::Value,
    #[serde(rename = "is_set_price")]
    pub is_set_price: ::serde_json::Value,
    #[serde(rename = "album_release_date")]
    pub album_release_date: String,
    #[serde(rename = "album_artist")]
    pub album_artist: ::serde_json::Value,
    #[serde(rename = "download_has_audio")]
    pub download_has_audio: bool,
    #[serde(rename = "type_id")]
    pub type_id: i64,
    #[serde(rename = "associated_license_id")]
    pub associated_license_id: ::serde_json::Value,
    #[serde(rename = "subscriber_only_published")]
    pub subscriber_only_published: bool,
    #[serde(rename = "album_art")]
    pub album_art: ::serde_json::Value,
    #[serde(rename = "options_title")]
    pub options_title: ::serde_json::Value,
    #[serde(rename = "featured_date")]
    pub featured_date: ::serde_json::Value,
    #[serde(rename = "download_art_id")]
    pub download_art_id: i64,
    #[serde(rename = "download_url")]
    pub download_url: String,
    #[serde(rename = "tax_rate")]
    pub tax_rate: ::serde_json::Value,
    pub currency: String,
    #[serde(rename = "selling_band_id")]
    pub selling_band_id: i64,
    #[serde(rename = "release_date")]
    pub release_date: ::serde_json::Value,
    pub options: ::serde_json::Value,
    pub sku: String,
    pub price: f64,
    #[serde(rename = "subscriber_only")]
    pub subscriber_only: ::serde_json::Value,
    #[serde(rename = "desc_pt1")]
    pub desc_pt1: String,
    #[serde(rename = "download_artist")]
    pub download_artist: String,
    pub arts: Vec<Art>,
    pub private: ::serde_json::Value,
    #[serde(rename = "new_desc_format")]
    pub new_desc_format: i64,
    #[serde(rename = "new_date")]
    pub new_date: String,
    pub label: ::serde_json::Value,
    #[serde(rename = "desc_pt2")]
    pub desc_pt2: ::serde_json::Value,
    pub description: String,
    #[serde(rename = "album_private")]
    pub album_private: ::serde_json::Value,
    #[serde(rename = "edition_size")]
    pub edition_size: ::serde_json::Value,
    #[serde(rename = "album_art_id")]
    pub album_art_id: i64,
    #[serde(rename = "band_id")]
    pub band_id: i64,
    pub upc: ::serde_json::Value,
    pub id: i64,
    #[serde(rename = "certified_seller")]
    pub certified_seller: i64,
    #[serde(rename = "album_id")]
    pub album_id: i64,
    #[serde(rename = "shipping_exception_mode")]
    pub shipping_exception_mode: ::serde_json::Value,
    #[serde(rename = "grid_index")]
    pub grid_index: i64,
    pub url: String,
    #[serde(rename = "quantity_limits")]
    pub quantity_limits: i64,
    #[serde(rename = "quantity_sold")]
    pub quantity_sold: ::serde_json::Value,
    #[serde(rename = "fulfillment_days")]
    pub fulfillment_days: i64,
    #[serde(rename = "download_type")]
    pub download_type: String,
    #[serde(rename = "album_publish_date")]
    pub album_publish_date: String,
    #[serde(rename = "album_title")]
    pub album_title: String,
    #[serde(rename = "download_title")]
    pub download_title: String,
    #[serde(rename = "download_id")]
    pub download_id: i64,
    pub origins: Vec<Origin>,
    pub title: String,
    #[serde(rename = "type_name")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Art {
    pub width: i64,
    #[serde(rename = "image_id")]
    pub image_id: i64,
    pub height: i64,
    #[serde(rename = "file_name")]
    pub file_name: String,
    pub index: i64,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    #[serde(rename = "quantity_available")]
    pub quantity_available: i64,
    pub quantity: ::serde_json::Value,
    #[serde(rename = "option_id")]
    pub option_id: i64,
    pub id: i64,
    #[serde(rename = "quantity_sold")]
    pub quantity_sold: ::serde_json::Value,
    #[serde(rename = "package_id")]
    pub package_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCapData {
    #[serde(rename = "streaming_limit")]
    pub streaming_limit: i64,
    #[serde(rename = "streaming_limits_enabled")]
    pub streaming_limits_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TralbumCollectInfo {
    #[serde(rename = "show_collect")]
    pub show_collect: bool,
    #[serde(rename = "show_wishlist_tooltip")]
    pub show_wishlist_tooltip: bool,
}
