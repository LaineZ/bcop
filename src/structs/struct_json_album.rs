#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonAlbum {
    pub title: String,
    pub art_id: String,
    pub artist: String,
}