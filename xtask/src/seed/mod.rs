use std::path::PathBuf;

use jiff::Timestamp;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use tokio::fs::read_to_string;

fn json_seed_file_path(file_name_root: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(&format!("../seed/{file_name_root}.json"));
    path
}

async fn parse_json_file<TTarget: DeserializeOwned>(
    file_name_root: &str,
) -> anyhow::Result<TTarget> {
    Ok(serde_json::from_str(
        &read_to_string(json_seed_file_path(file_name_root)).await?,
    )?)
}

pub async fn seed() -> anyhow::Result<()> {
    let planets: Vec<Planet> = parse_json_file("planets").await?;
    unimplemented!()
}

#[serde_as]
#[derive(Deserialize)]
struct Planet {
    edited: Timestamp,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    climates: Vec<String>,
}
