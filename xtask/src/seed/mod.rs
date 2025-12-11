use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use jiff::Timestamp;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
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
    created: Timestamp,
    #[serde(rename = "climate")]
    #[serde_as(as = "StringWithSeparator::<CommaSpaceSeparator, String>")]
    climates: Vec<String>,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    surface_water: Option<f64>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    diameter: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    rotation_period: Option<u32>,
}

#[derive(Debug)]
struct Planet {
    edited: Timestamp,
    created: Timestamp,
    climates: Vec<String>,
    id: u32,
    name: String,
    surface_water: Option<f64>,
    diameter: Option<u32>,
    rotation_period: Option<u32>,
}

impl From<PlanetNested> for Planet {
    fn from(value: PlanetNested) -> Self {
        Self {
            edited: value.fields.edited,
            created: value.fields.created,
            climates: value.fields.climates,
            id: value.id,
            name: value.fields.name,
            surface_water: value.fields.surface_water,
            diameter: value.fields.diameter,
            rotation_period: value.fields.rotation_period,
        }
    }
}

struct CommaSpaceSeparator;

impl Separator for CommaSpaceSeparator {
    fn separator() -> &'static str {
        ", "
    }
}

// https://github.com/serde-rs/json/issues/317#issuecomment-300251188
fn deserialize_from_str<'de, TTarget, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<TTarget, TDeserializer::Error>
where
    TTarget: FromStr,
    TTarget::Err: Display,
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    TTarget::from_str(&str).map_err(de::Error::custom)
}

fn deserialize_from_str_or_unknown<'de, TTarget, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<TTarget>, TDeserializer::Error>
where
    TTarget: FromStr,
    TTarget::Err: Display,
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(match &*str {
        "unknown" => None,
        str => Some(TTarget::from_str(str).map_err(de::Error::custom)?),
    })
}
