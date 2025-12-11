use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use jiff::Timestamp;
use jiff_sqlx::ToSqlx;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use shared::get_db_pool;
use sqlx::{Pool, Postgres, QueryBuilder};
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

    seed_planets(&db_pool).await?;
    seed_people(&db_pool).await?;
    unimplemented!()
}

async fn seed_planets(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let planets: Vec<Planet> = parse_json_file::<Vec<PlanetNested>>("planets")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("planets: {planets:#?}");

    let mut query_builder = QueryBuilder::new("INSERT INTO planets (id, edited, created, name, surface_water, diameter, rotation_period, gravity, orbital_period, population)");
    query_builder.push_values(planets, |mut builder, planet| {
        builder
            .push_bind(i32::try_from(planet.id).unwrap())
            .push_bind(planet.edited.to_sqlx())
            .push_bind(planet.created.to_sqlx())
            .push_bind(planet.name)
            .push_bind(planet.surface_water)
            .push_bind(
                planet
                    .diameter
                    .map(|diameter| i32::try_from(diameter).unwrap()),
            )
            .push_bind(
                planet
                    .rotation_period
                    .map(|rotation_period| i32::try_from(rotation_period).unwrap()),
            )
            .push_bind(planet.gravity)
            .push_bind(
                planet
                    .orbital_period
                    .map(|orbital_period| i32::try_from(orbital_period).unwrap()),
            )
            .push_bind(planet.population);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    Ok(())
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

async fn seed_people(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let people: Vec<Person> = parse_json_file::<Vec<PersonNested>>("people")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("people: {people:#?}");

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonNested {
    fields: PersonNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonNestedFields {
    edited: Timestamp,
    created: Timestamp,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    gender: Option<Gender>,
    #[serde(
        rename = "skin_color",
        deserialize_with = "deserialize_comma_separated_or_unknown"
    )]
    skin_colors: Option<Vec<SkinColor>>,
    #[serde(
        rename = "hair_color",
        deserialize_with = "deserialize_comma_separated_or_unknown"
    )]
    hair_colors: Option<Vec<HairColor>>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    height: Option<u32>,
    #[serde(
        rename = "eye_color",
        deserialize_with = "deserialize_comma_separated_or_unknown"
    )]
    eye_colors: Option<Vec<EyeColor>>,
    #[serde(deserialize_with = "deserialize_from_str_strip_commas_or_unknown")]
    mass: Option<f64>,
    homeworld: u32,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    birth_year: Option<String>,
}

#[derive(Debug)]
struct Person {
    edited: Timestamp,
    created: Timestamp,
    id: u32,
    name: String,
    gender: Option<Gender>,
    skin_colors: Option<Vec<SkinColor>>,
    hair_colors: Option<Vec<HairColor>>,
    height: Option<u32>,
    eye_colors: Option<Vec<EyeColor>>,
    mass: Option<f64>,
    homeworld: u32,
    birth_year: Option<String>,
}

impl From<PersonNested> for Person {
    fn from(value: PersonNested) -> Self {
        Self {
            edited: value.fields.edited,
            created: value.fields.created,
            id: value.id,
            name: value.fields.name,
            gender: value.fields.gender,
            skin_colors: value.fields.skin_colors,
            hair_colors: value.fields.hair_colors,
            height: value.fields.height,
            eye_colors: value.fields.eye_colors,
            mass: value.fields.mass,
            homeworld: value.fields.homeworld,
            birth_year: value.fields.birth_year,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Gender {
    Male,
    Female,
    Hermaphrodite,
}

impl FromStr for Gender {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            "hermaphrodite" => Ok(Self::Hermaphrodite),
            _ => Err("Unknown gender"),
        }
    }
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "kebab-case")]
enum SkinColor {
    Caucasian,
    Black,
    Asian,
    Hispanic,
    Grey,
    Fair,
    Gold,
    White,
    Blue,
    Light,
    Red,
    Green,
    GreenTan,
    Brown,
    Pale,
    Metal,
    Dark,
    BrownMottle,
    MottledGreen,
    Orange,
    Yellow,
    Tan,
    Silver,
}

impl FromStr for SkinColor {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "caucasian" => Ok(Self::Caucasian),
            "black" => Ok(Self::Black),
            "asian" => Ok(Self::Asian),
            "hispanic" => Ok(Self::Hispanic),
            "gray" | "grey" => Ok(Self::Grey),
            "fair" => Ok(Self::Fair),
            "gold" => Ok(Self::Gold),
            "white" => Ok(Self::White),
            "blue" => Ok(Self::Blue),
            "light" => Ok(Self::Light),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "green-tan" => Ok(Self::GreenTan),
            "brown" => Ok(Self::Brown),
            "pale" => Ok(Self::Pale),
            "metal" => Ok(Self::Metal),
            "dark" => Ok(Self::Dark),
            "brown mottle" => Ok(Self::BrownMottle),
            "mottled green" => Ok(Self::MottledGreen),
            "orange" => Ok(Self::Orange),
            "yellow" => Ok(Self::Yellow),
            "tan" => Ok(Self::Tan),
            "silver" => Ok(Self::Silver),
            _ => Err("Unknown skin color"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum HairColor {
    #[serde(alias = "blond")]
    Blonde,
    Brown,
    Black,
    Red,
    Grey,
    Auburn,
    White,
}

impl FromStr for HairColor {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "blonde" | "blond" => Ok(Self::Blonde),
            "brown" => Ok(Self::Brown),
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "grey" => Ok(Self::Grey),
            "auburn" => Ok(Self::Auburn),
            "white" => Ok(Self::White),
            _ => Err("Unknown hair color"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum EyeColor {
    Brown,
    Blue,
    Green,
    Hazel,
    Grey,
    Amber,
    Yellow,
    Golden,
    Red,
    Black,
    BlueGray,
    Orange,
    Pink,
    Gold,
    White,
}

impl FromStr for EyeColor {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "brown" => Ok(Self::Brown),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "hazel" => Ok(Self::Hazel),
            "grey" => Ok(Self::Grey),
            "amber" => Ok(Self::Amber),
            "yellow" => Ok(Self::Yellow),
            "golden" => Ok(Self::Golden),
            "red" => Ok(Self::Red),
            "black" => Ok(Self::Black),
            "blue-gray" => Ok(Self::BlueGray),
            "orange" => Ok(Self::Orange),
            "pink" => Ok(Self::Pink),
            "gold" => Ok(Self::Gold),
            "white" => Ok(Self::White),
            _ => Err("Unknown eye color"),
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
        "unknown" | "n/a" | "none" => None,
        str => Some(TTarget::from_str(str).map_err(de::Error::custom)?),
    })
}

fn deserialize_comma_separated_or_unknown<'de, TTarget, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<Vec<TTarget>>, TDeserializer::Error>
where
    TTarget: FromStr,
    TTarget::Err: Display,
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(match &*str {
        "unknown" | "n/a" | "none" => None,
        str => Some(
            str.split(", ")
                .map(|chunk| match regex!(r#"^[^,]+$"#).is_match(chunk) {
                    true => TTarget::from_str(chunk).map_err(|_| ".from_str() failed"),
                    false => Err("Unexpected format"),
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(de::Error::custom)?,
        ),
    })
}

fn deserialize_from_str_strip_commas_or_unknown<'de, TTarget, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<TTarget>, TDeserializer::Error>
where
    TTarget: FromStr,
    TTarget::Err: Display,
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(match &*str {
        "unknown" | "n/a" | "none" => None,
        str => Some(TTarget::from_str(&str.replace(",", "")).map_err(de::Error::custom)?),
    })
}
