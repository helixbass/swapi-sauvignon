use std::str::FromStr;

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Type};
use strum::{Display, VariantNames};

pub async fn get_db_pool() -> anyhow::Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://swapi_sauvignon:password@localhost/swapi_sauvignon")
        .await?;

    Ok(db_pool)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum SpeciesClassification {
    Mammal,
    Artificial,
    Sentient,
    Gastropod,
    Reptile,
    Amphibian,
    Insectoid,
    Reptilian,
}

impl FromStr for SpeciesClassification {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mammal" | "mammals" => Ok(Self::Mammal),
            "artificial" => Ok(Self::Artificial),
            "sentient" => Ok(Self::Sentient),
            "gastropod" => Ok(Self::Gastropod),
            "reptile" => Ok(Self::Reptile),
            "amphibian" => Ok(Self::Amphibian),
            "insectoid" => Ok(Self::Insectoid),
            "reptilian" => Ok(Self::Reptilian),
            _ => Err("Unknown species classification"),
        }
    }
}
