use crate::{config::anime_config::AnimeByFansub, errors::error::AppError};

use super::torrent_metadata::{TorrentMetadata};

const SEPARATOR: &str = " - ";


/// Returns the number of episode if the torrent title matches the anime entry
///
/// Result is -1 if the torrent doesn't match the anime entry
pub fn torrent_episode_if_matches_anime_entry(
    torrent_title: &str,
    fansub_entry: &AnimeByFansub,
) -> Result<TorrentMetadata, AppError> {
    let fansub = &format!("[{}]", fansub_entry.fansub);
    let matches_fansub = torrent_title.starts_with(fansub);

    if !matches_fansub {
        return Err(AppError::FansubNotMatching);
    }

    let torrent_name_without_fansub = torrent_title.trim_start_matches(fansub) 
        .trim();

    let starts_with_anime_title = torrent_name_without_fansub.starts_with(fansub_entry.name.as_str());

    if !starts_with_anime_title {
        return Err(AppError::AnimeTitleNotMatching);
    }

    let episode_and_metadata = torrent_name_without_fansub
        .trim_start_matches(fansub_entry.name.as_str())
        .trim_start_matches(SEPARATOR) // Remove possible separators between anime name and the rest of data (such as eps or metadata)
        .trim();

    let episode = get_episode(episode_and_metadata)?;
    let tags = get_metadata_tags(episode_and_metadata);

    for keyword in &fansub_entry.keywords {
        if !tags.contains(keyword) {
            return Err(AppError::MissingTag);
        }
    }

    let torrent_metadata = TorrentMetadata { 
        episode,
        tags
    };

    return Ok(torrent_metadata);
}


fn get_episode(episode_and_metadata: &str) -> Result<f64, AppError> {
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

    let episode = episode_string
        .parse::<f64>()
        .map_err(|_| AppError::ParseTorrentTitleError)?;

    return Ok(episode)
}

fn get_metadata_tags(episode_and_metadata: &str) -> Vec<String> {
    let first_tag_position = episode_and_metadata
        .chars()
        .position(|ch| ch == '(' || ch == '[');

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
        }
        None => {
            return vec![];
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_episode_with_season() -> Result<(), AppError>  {
        let episode = get_episode(
            "S01E04 [TAG][TAG2]"
        )?;
        
        assert_eq!(episode, 4.0);

        return Ok(())
    }

    #[test]
    fn test_get_metadata_tags() -> Result<(), AppError>  {
        let metadata = get_metadata_tags(
            "S01E04 [TAG][TAG2]"
        );
        
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0], "[TAG]");
        assert_eq!(metadata[1], "[TAG2]");

        return Ok(())
    }

    #[test]
    fn test_torrent_episode_if_matches_anime_entry() -> Result<(), AppError>  {
        let anime_by_fansub = AnimeByFansub {
            name: "My title".to_string(),
            fansub: "FanSub".to_string(),
            keywords: vec!["[1080p]".to_string()],
        };

        let metadata = torrent_episode_if_matches_anime_entry(
            "[FanSub] My title S02E02 [1080p][HEVC 10bit x265][AAC]",
            &anime_by_fansub            
        )?;
        

        assert_eq!(metadata.episode, 2.0);

        return Ok(())
    }

    #[test]
    fn test_torrent_episode_if_matches_anime_entry_with_separator() -> Result<(), AppError> {
        let anime_by_fansub = AnimeByFansub {
            name: "My title - Part 2".to_string(),
            fansub: "FanSub".to_string(),
            keywords: vec!["[1080p]".to_string(), "[HEVC]".to_string()],
        };

        let metadata = torrent_episode_if_matches_anime_entry(
            "[FanSub] My title - Part 2 - 02 [1080p][HEVC]",
            &anime_by_fansub            
        )?;
        

        assert_eq!(metadata.episode, 2.0);

        return Ok(())
    }

    #[test]
    fn test_torrent_episode_if_matches_anime_entry_with_no_matching_tag() {
        let anime_by_fansub = AnimeByFansub {
            name: "My title - Part 2".to_string(),
            fansub: "FanSub".to_string(),
            keywords: vec!["[1080p]".to_string(), "[HEVC]".to_string()],
        };

        let metadata = torrent_episode_if_matches_anime_entry(
            "[FanSub] My title - Part 2 - 02 [720p][HEVC]",
            &anime_by_fansub            
        );
        
        match metadata {
            Ok(_) => assert!(false, "Result is OK but should be Err"),
            Err(error) => {
                if !matches!(error, AppError::MissingTag) {
                    assert!(false, "Different error than the expected MissingTag")
                }
            },
        }

        

    }
}

