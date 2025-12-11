use std::path::PathBuf;

use jiff::Timestamp;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{formats::Separator, serde_as, StringWithSeparator};
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
    let planets: Vec<Planet> = parse_json_file::<Vec<PlanetNested>>("planets")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("planets: {planets:#?}");
    unimplemented!()
}

#[derive(Debug, Deserialize)]
struct PlanetNested {
    fields: PlanetNestedFields,
    #[serde(rename = "pk")]
    id: u32,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct PlanetNestedFields {
    edited: Timestamp,
    #[serde(rename = "climate")]
    #[serde_as(as = "StringWithSeparator::<CommaSpaceSeparator, String>")]
    climates: Vec<String>,
}

#[derive(Debug)]
struct Planet {
    edited: Timestamp,
    climates: Vec<String>,
    id: u32,
}

impl From<PlanetNested> for Planet {
    fn from(value: PlanetNested) -> Self {
        Self {
            edited: value.fields.edited,
            climates: value.fields.climates,
            id: value.id,
        }
    }
}

struct CommaSpaceSeparator;

impl Separator for CommaSpaceSeparator {
    fn separator() -> &'static str {
        ", "
    }
}
