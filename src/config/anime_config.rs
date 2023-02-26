use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeConfig {
    pub fansubs: Vec<AnimeByFansub>,

    #[serde(rename = "storageName")]
    pub storage_name: String,

    #[serde(rename = "downloadPath")]
    pub download_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeByFansub {
    pub name: String,

    pub fansub: String,

    pub keywords: Vec<String>,

    // #[serde(rename = "item")]
    // pub item: Vec<Item>,
}
