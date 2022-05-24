use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Res {
    code: u8,
    more: bool,
    songs: Vec<Song>,
}

#[derive(Deserialize, Serialize)]
struct Song {
    name: String,
    id: u32,
    al: Album,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Album {
    id: u32,
    name: String,
    pic_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://olbb.vercel.app/";
    let singer_id = "15345";
    let res = reqwest::get(format!("{}/artist/top/song?id={}", base_url, singer_id).as_str())
        .await?
        .text()
        .await?;
    let res: Res = serde_json::from_str(res.as_str())?;
    let mut file = fs::File::create("data.json")?;
    file.write_all(serde_json::to_string_pretty(&res.songs).unwrap().as_bytes()).unwrap();
    Ok(())
}