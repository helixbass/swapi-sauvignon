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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum SpeciesDesignation {
    Sentient,
    Reptilian,
}

impl FromStr for SpeciesDesignation {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sentient" => Ok(Self::Sentient),
            "reptilian" => Ok(Self::Reptilian),
            _ => Err("Unknown species designation"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    GalacticBasic,
    Shyriiwook,
    Huttese,
    Dosh,
    MonCalamarian,
    Ewokese,
    Sullutese,
    Neimoidia,
    GunganBasic,
    Toydarian,
    Dugese,
    TwiLeki,
    Aleena,
    Vulpterish,
    Xextese,
    Tundan,
    Cerean,
    Nautila,
    Zabraki,
    Iktotchese,
    Quermian,
    KelDor,
    Chagria,
    Geonosian,
    Mirialan,
    Clawdite,
    Besalisk,
    Kaminoan,
    Skakoan,
    Muun,
    Togruti,
    Kaleesh,
    Utapese,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Galactic Basic" | "Galatic Basic" | "Galactic basic" => Ok(Self::GalacticBasic),
            "Shyriiwook" => Ok(Self::Shyriiwook),
            "Huttese" => Ok(Self::Huttese),
            "Dosh" => Ok(Self::Dosh),
            "Mon Calamarian" => Ok(Self::MonCalamarian),
            "Ewokese" => Ok(Self::Ewokese),
            "Sullutese" => Ok(Self::Sullutese),
            "Neimoidia" => Ok(Self::Neimoidia),
            "Gungan basic" => Ok(Self::GunganBasic),
            "Toydarian" => Ok(Self::Toydarian),
            "Dugese" => Ok(Self::Dugese),
            "Twi'leki" => Ok(Self::TwiLeki),
            "Aleena" => Ok(Self::Aleena),
            "vulpterish" => Ok(Self::Vulpterish),
            "Xextese" => Ok(Self::Xextese),
            "Tundan" => Ok(Self::Tundan),
            "Cerean" => Ok(Self::Cerean),
            "Nautila" => Ok(Self::Nautila),
            "Zabraki" => Ok(Self::Zabraki),
            "Iktotchese" => Ok(Self::Iktotchese),
            "Quermian" => Ok(Self::Quermian),
            "Kel Dor" => Ok(Self::KelDor),
            "Chagria" => Ok(Self::Chagria),
            "Geonosian" => Ok(Self::Geonosian),
            "Mirialan" => Ok(Self::Mirialan),
            "Clawdite" => Ok(Self::Clawdite),
            "besalisk" => Ok(Self::Besalisk),
            "Kaminoan" => Ok(Self::Kaminoan),
            "Skakoan" => Ok(Self::Skakoan),
            "Muun" => Ok(Self::Muun),
            "Togruti" => Ok(Self::Togruti),
            "Kaleesh" => Ok(Self::Kaleesh),
            "Utapese" => Ok(Self::Utapese),
            _ => Err("Unknown language"),
        }
    }
}
