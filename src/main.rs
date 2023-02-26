mod config;
mod downloaded_data;
mod errors;
mod transmission;
mod webservice;

use config::anime_config::{AnimeByFansub, AnimeConfig};
use config::reader_config;
use downloaded_data::downloaded_anime_repository;
use errors::error::AppError;
use std::vec;
use std::{thread, time::Duration};
use webservice::nyaa_rss::{AnimeTorrent, NyaaRss};
use webservice::rss_reader;

const SLEEP_TIME_IN_MINS: u64 = 2;
const EPISODE_NOT_FOUND: f64 = -1.0;

fn main() {
    loop {
        match run_task() {
            Ok(_) => {
                println!("Task executed successfully");
            }
            Err(error) => {
                println!("Task failed due to {:#?}", error);
            }
        }

        println!("Sleeping for {} minutes", SLEEP_TIME_IN_MINS);
        thread::sleep(Duration::from_secs(SLEEP_TIME_IN_MINS * 60));
    }
}

fn run_task() -> Result<(), AppError> {
    let config = reader_config::get_config()?;
    let content = rss_reader::get_rss_data()?;

    match download_anime_if_required(&content, &config) {
        Ok(_) => {
            println!("download_anime_if_required completed successfully");
        }
        Err(error) => {
            println!("download_anime_if_required failed due to {:#?}", error);
        }
    };

    Ok(())
}

fn download_anime_if_required(feed: &NyaaRss, config: &Vec<AnimeConfig>) -> Result<(), AppError> {
    let items = &feed.channel.item;

    for anime_torrent in items {
        let mut downloaded_anime_data = downloaded_anime_repository::load_downloaded_anime_data()?;

        for anime in config {
            for fansub_entry in &anime.fansubs {
                match torrent_episode_if_matches_anime_entry(&anime_torrent, fansub_entry) {
                    Ok(episode) => {
                        // Episode not found, ignore it...
                        if episode == EPISODE_NOT_FOUND {
                            println!(
                                "Torrent \"{}\" does not match any specified data",
                                anime_torrent.title
                            )
                        } else {
                            println!("Torrent {} matches entry! Checking if it has been already downloaded...", anime_torrent.title);

                            let default_episodes_list: Vec<f64> = vec![];
                            let mut episodes = downloaded_anime_data
                                .get(&anime.storage_name)
                                .unwrap_or(&default_episodes_list)
                                .to_vec();

                            if episodes.contains(&episode) {
                                println!(
                                    "Episode {} was already downloaded for {}",
                                    episode, anime.storage_name
                                );
                            } else {
                                println!(
                                    "Sending torrent of {} - {} to transmission",
                                    anime.storage_name, episode
                                );

                                transmission::commands::send(
                                    anime.download_path.as_str(),
                                    anime_torrent.link.as_str(),
                                )?;

                                episodes.push(episode);

                                downloaded_anime_data
                                    .insert(anime.storage_name.to_string(), episodes);

                                downloaded_anime_repository::save_downloaded_anime(
                                    &downloaded_anime_data,
                                )?;
                            }
                        }
                    }
                    Err(error) => {
                        println!("{:#?}", error);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Returns the number of episode if the torrent title matches the anime entry
///
/// Result is -1 if the torrent doesn't match the anime entry
fn torrent_episode_if_matches_anime_entry(
    torrent: &AnimeTorrent,
    fansub_entry: &AnimeByFansub,
) -> Result<f64, AppError> {
    let torrent_title = torrent.title.clone();

    let fansub = format!("[{}] ", fansub_entry.fansub);

    let same_fansub = torrent_title.starts_with(fansub.as_str());

    if same_fansub {
        let torrent_name_without_fansub = torrent_title.trim_start_matches(fansub.as_str());
        let torrent_name_split: Vec<&str> = torrent_name_without_fansub.split(" - ").collect();

        if torrent_name_split.len() == 2 {
            let anime_name = torrent_name_split
                .get(0)
                .ok_or(AppError::ParseTorrentTitleError)?;
            let episode_and_metadata = torrent_name_split
                .get(1)
                .ok_or(AppError::ParseTorrentTitleError)?;

            let mut episode_string = episode_and_metadata
                .chars()
                .take_while(|&ch| ch != '[' && ch != '(')
                .collect::<String>()
                .trim()
                .to_string();

            if let Some(ep_identifier_position) = episode_string.find('E') {
                episode_string = episode_string
                    .chars()
                    .skip(ep_identifier_position + 1)
                    .collect();
            }

            let metadata = get_metadata_tags(episode_and_metadata);

            let episode = episode_string
                .parse::<f64>()
                .map_err(|_| AppError::ParseTorrentTitleError)?;

            let matching_keywords = fansub_entry
                .keywords
                .iter()
                .filter(|&keyword| metadata.contains(keyword))
                .count();
            let contains_all_keywords = matching_keywords == fansub_entry.keywords.len();

            if *anime_name == fansub_entry.name && contains_all_keywords {
                return Ok(episode);
            }
        }
    }

    return Ok(EPISODE_NOT_FOUND);
}

fn get_metadata_tags(episode_and_metadata: &str) -> Vec<String> {
    let first_tag_position = episode_and_metadata.chars().position(| ch | ch == '(' || ch == '[');

    match first_tag_position {
        Some(position) => {
            let metadata = &episode_and_metadata[position..episode_and_metadata.len()];

            let mut metadata_tags: Vec<String> = vec![];
            let mut current_metadata_tag = String::new();
        
            for ch in metadata.chars() {
                current_metadata_tag.push(ch);
        
                if ch == ']' || ch == ')' {
                    metadata_tags.push(current_metadata_tag.to_string());
                    current_metadata_tag.clear();
                }
            }
        
            // In case the latest tag doesn't have use [] or ()
            if !current_metadata_tag.is_empty() {
                metadata_tags.push(current_metadata_tag.to_string());
            }
        
            metadata_tags
        },
        None => {
            return vec![];
        },
    }   
}
