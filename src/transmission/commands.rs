use std::process::Command;

use crate::errors::error::AppError;


pub fn send(download_path: &str, torrent_link: &str) -> Result<(), AppError> {
    Command::new("transmission-remote")
        .arg("-w")
        .arg(download_path)
        .arg("-a")
        .arg(torrent_link)
        .spawn()
        .map_err(|error| AppError::RunCommandError(error))?;

    Ok(())
}