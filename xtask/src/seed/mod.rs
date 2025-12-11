use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use jiff::Timestamp;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use shared::get_db_pool;
use sqlx::{Pool, Postgres};
use squalid::regex;
use tokio::fs::read_to_string;

fn workspace_root_directory() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path
}

fn json_seed_file_path(file_name_root: &str) -> PathBuf {
    let mut path = workspace_root_directory();
    path.push(&format!("seed/{file_name_root}.json"));
    path
}

async fn parse_json_file<TTarget: DeserializeOwned>(
    file_name_root: &str,
) -> anyhow::Result<TTarget> {
    Ok(serde_json::from_str(
        &read_to_string(json_seed_file_path(file_name_root)).await?,
    )?)
}

fn sql_file_path(file_name_root: &str) -> PathBuf {
    let mut path = workspace_root_directory();
    path.push(&format!("sql/{file_name_root}.sql"));
    path
}

async fn create_tables(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let sql = read_to_string(sql_file_path("create_tables")).await?;
    sqlx::query(&sql).execute(db_pool).await?;
    Ok(())
}

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await?;
    create_tables(&db_pool).await?;
    let planets: Vec<Planet> = parse_json_file::<Vec<PlanetNested>>("planets")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("planets: {planets:#?}");
    unimplemented!()
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanetNested {
    fields: PlanetNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanetNestedFields {
    edited: Timestamp,
    created: Timestamp,
    #[serde(
        rename = "climate",
        deserialize_with = "deserialize_comma_separated_or_unknown"
    )]
    climates: Option<Vec<String>>,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    surface_water: Option<f64>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    diameter: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    rotation_period: Option<u32>,
    #[serde(
        rename = "terrain",
        deserialize_with = "deserialize_comma_separated_or_unknown"
    )]
    terrains: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    gravity: Option<String>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    orbital_period: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    population: Option<f64>,
}

#[derive(Debug)]
struct Planet {
    edited: Timestamp,
    created: Timestamp,
    climates: Option<Vec<String>>,
    id: u32,
    name: String,
    surface_water: Option<f64>,
    diameter: Option<u32>,
    rotation_period: Option<u32>,
    terrains: Option<Vec<String>>,
    gravity: Option<String>,
    orbital_period: Option<u32>,
    population: Option<f64>,
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
            terrains: value.fields.terrains,
            gravity: value.fields.gravity,
            orbital_period: value.fields.orbital_period,
            population: value.fields.population,
        }
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

fn deserialize_comma_separated_or_unknown<'de, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<Vec<String>>, TDeserializer::Error>
where
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(match &*str {
        "unknown" => None,
        str => Some(
            str.split(", ")
                .map(|chunk| match regex!(r#"^[^,]+$"#).is_match(chunk) {
                    true => Ok(chunk.to_owned()),
                    false => Err("Unexpected format"),
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(de::Error::custom)?,
        ),
    })
}
