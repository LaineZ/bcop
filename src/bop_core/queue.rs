pub struct Track {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub url: String,
}

pub struct PlayQueue {
    pub tracks: Vec<Track>,
    pub shuffle: bool,
}
