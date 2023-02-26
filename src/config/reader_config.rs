use crate::errors::error::AppError;

use super::anime_config::AnimeConfig;
use std::fs;

pub fn get_config() -> Result<Vec<AnimeConfig>, AppError> {
    let text = read_config()?;

    serde_json::from_str::<Vec<AnimeConfig>>(text.as_str())
        .map_err(|error| AppError::ParseJsonError(error))
}

fn read_config() -> Result<String, AppError> {
    let file_path = "anime_to_download.json";

    fs::read_to_string(file_path).map_err(|error| AppError::FileReadError(error))
}
