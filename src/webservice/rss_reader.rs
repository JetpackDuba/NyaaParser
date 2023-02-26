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
//     let data = r#"<rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:nyaa="https://nyaa.si/xmlns/nyaa" version="2.0">
// 	<channel>
// 		<title>Nyaa - Home - Torrent File RSS</title>
// 		<description>RSS Feed for Home</description>
// 		<link>https://nyaa.si/</link>
// 		<atom:link href="https://nyaa.si/?page=rss" rel="self" type="application/rss+xml" />
// 		<item>
// 			<title>[DKB] Fumetsu no Anata e - S02E17 [1080p][HEVC x265 10bit][Multi-Subs][weekly]</title>
// 			<link>https://nyaa.si/download/1621349.torrent</link>
// 			<guid isPermaLink="true">https://nyaa.si/view/1621349</guid>
// 			<pubDate>Fri, 06 Jan 2023 20:47:55 -0000</pubDate>

// 			<nyaa:seeders>0</nyaa:seeders>
// 			<nyaa:leechers>2</nyaa:leechers>
// 			<nyaa:downloads>0</nyaa:downloads>
// 			<nyaa:infoHash>244841ce0098cb512dec286074c9d000b13bea00</nyaa:infoHash>
// 			<nyaa:categoryId>1_3</nyaa:categoryId>
// 			<nyaa:category>Anime - Non-English-translated</nyaa:category>
// 			<nyaa:size>210.3 MiB</nyaa:size>
// 			<nyaa:comments>0</nyaa:comments>
// 			<nyaa:trusted>No</nyaa:trusted>
// 			<nyaa:remake>No</nyaa:remake>
// 			<description><![CDATA[<a href="https://nyaa.si/view/1621349">#1621349 | [BakeSubs] Spy Kyoushitsu - 01 [720p][EB5EFDC5].mkv</a> | 210.3 MiB | Anime - Non-English-translated | 244841CE0098CB512DEC286074C9D000B13BEA00]]></description>
// 		</item>
// 	</channel>
// </rss>"#;
    // Ok(data.to_string())
}

fn parse_content(content: &str) -> Result<NyaaRss, AppError> {
    from_str::<NyaaRss>(content).map_err(|_| AppError::ParseXmlError)
}
