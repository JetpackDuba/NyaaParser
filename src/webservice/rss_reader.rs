use quick_xml::de::from_str;

use crate::errors::error::AppError;

use super::nyaa_rss::NyaaRss;

const URL: &str = "https://nyaa.si/?page=rss";

pub fn get_rss_data() -> Result<NyaaRss, AppError> {
    let text = get_data_from_remote()?;

    parse_content(text.as_str())
}

fn get_data_from_remote() -> Result<String, AppError> {
    let request = reqwest::blocking::get(URL).map_err(|error| AppError::GetNetworkData(error))?;

    request.text().map_err(|_| AppError::ParseXmlError)
}

fn parse_content(content: &str) -> Result<NyaaRss, AppError> {
    from_str::<NyaaRss>(content).map_err(|_| AppError::ParseXmlError)
}
