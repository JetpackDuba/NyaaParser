use std::io;
use serde_json;
use reqwest;

#[derive(Debug)]
pub enum AppError {
    FileWriteError(io::Error),
    FileReadError(io::Error),
    ParseTorrentTitleError,
    AnimeTitleNotMatching,
    MissingTag,
    FansubNotMatching,
    ParseJsonError(serde_json::Error),
    SerializeError(serde_json::Error),
    GetNetworkData(reqwest::Error),
    ParseXmlError,
    RunCommandError(io::Error),
}