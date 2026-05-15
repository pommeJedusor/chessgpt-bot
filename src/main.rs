use std::{collections::HashMap, time::Duration};

use tokio::time::sleep;

const URL: &str = "https://chessgpt.ai/api/leaderboard";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("nickname", "nya >:3 🏳️‍⚧️");
    map.insert("result", "win");

    let client = reqwest::Client::new();

    loop {
        let res = client.post(URL).json(&map).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {}
            reqwest::StatusCode::TOO_MANY_REQUESTS => sleep(Duration::from_secs(5)).await,
            _ => eprint!("{:?}", res.status()),
        }
    }
}
