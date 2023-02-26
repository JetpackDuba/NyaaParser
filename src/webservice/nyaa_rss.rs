use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NyaaRss {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "item")]
    pub item: Vec<AnimeTorrent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeTorrent {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "link")]
    pub link: String,
}
