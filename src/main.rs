mod config;
mod downloaded_data;
mod errors;
mod torrent_parser;
mod transmission;
mod webservice;

use config::anime_config::{AnimeConfig};
use config::reader_config;
use downloaded_data::downloaded_anime_repository;
use errors::error::AppError;
use std::vec;
use std::{thread, time::Duration};
use torrent_parser::torrent_parser::torrent_episode_if_matches_anime_entry;
use webservice::nyaa_rss::{NyaaRss};
use webservice::rss_reader;

const SLEEP_TIME_IN_MINS: u64 = 2;

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
                match torrent_episode_if_matches_anime_entry(
                    anime_torrent.title.as_str(),
                    fansub_entry,
                ) {
                    Ok(torrent_metadata) => {
                        let episode = torrent_metadata.episode;

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

                            downloaded_anime_data.insert(anime.storage_name.to_string(), episodes);

                            downloaded_anime_repository::save_downloaded_anime(
                                &downloaded_anime_data,
                            )?;
                        }
                    }
                    Err(error) => {
                        println!("\"{}\" {:#?}", anime_torrent.title, error);
                    }
                }
            }
        }
    }

    Ok(())
}
