use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadedAnime{
    pub name: String,
    pub episodes_downloaded: Vec<String>,
}