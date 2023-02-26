use crate::errors::error::AppError;

use std::{collections::HashMap, fs, path};

const FILE_PATH: &str = "anime_db.json";

pub fn load_downloaded_anime_data() -> Result<HashMap<String, Vec<f64>>, AppError> {
    if !path::Path::new(FILE_PATH).exists() {
        fs::File::create(FILE_PATH).map_err(|error| AppError::FileWriteError(error))?;
    }

    let text = read_config()?;

    if text.is_empty() {
        Ok(HashMap::new())
    } else {
        parse_content(text.as_str())
    }
}

fn read_config() -> Result<String, AppError> {
    fs::read_to_string(FILE_PATH).map_err(|error| AppError::FileReadError(error))
}

fn parse_content(text: &str) -> Result<HashMap<String, Vec<f64>>, AppError> {
    serde_json::from_str::<HashMap<String, Vec<f64>>>(text)
        .map_err(|error| AppError::ParseJsonError(error))
}

pub fn save_downloaded_anime(downloaded_anime_data: &HashMap<String, Vec<f64>>) -> Result<(), AppError> {
    let json_str = content_to_json_str(&downloaded_anime_data)?;
    write_config(json_str.as_str())?;

    Ok(())
}

fn write_config(json_str: &str) -> Result<(), AppError> {
    fs::write(FILE_PATH, json_str).map_err(|error| AppError::FileWriteError(error))
}

fn content_to_json_str(downloaded_anime_data: &HashMap<String, Vec<f64>>) -> Result<String, AppError> {
    serde_json::to_string(downloaded_anime_data)
    .map_err(|error| AppError::SerializeError(error))
}
