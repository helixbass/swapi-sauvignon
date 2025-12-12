use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

use jiff::Timestamp;
use jiff_sqlx::ToSqlx;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use shared::get_db_pool;
use sqlx::{Pool, Postgres, QueryBuilder, Type};
use squalid::{_d, fancy_regex, regex};
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
    for command in sql.split("\n\n") {
        sqlx::query(command).execute(db_pool).await?;
    }
    Ok(())
}

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await?;

    create_tables(&db_pool).await?;

    seed_planets(&db_pool).await?;
    let people_species = seed_species(&db_pool).await?;
    seed_people(&db_pool, &people_species).await?;
    seed_transports(&db_pool).await?;
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
    query_builder.push_values(&planets, |mut builder, planet| {
        builder
            .push_bind(i32::try_from(planet.id).unwrap())
            .push_bind(planet.edited.to_sqlx())
            .push_bind(planet.created.to_sqlx())
            .push_bind(planet.name.clone())
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
            .push_bind(planet.gravity.clone())
            .push_bind(
                planet
                    .orbital_period
                    .map(|orbital_period| i32::try_from(orbital_period).unwrap()),
            )
            .push_bind(planet.population);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder = QueryBuilder::new("INSERT INTO planet_climates (planet_id, climate)");
    query_builder.push_values(
        planets.iter().flat_map(|planet| {
            planet
                .climates
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|climate| (planet.id, climate))
        }),
        |mut builder, (planet_id, climate)| {
            builder
                .push_bind(i32::try_from(planet_id).unwrap())
                .push_bind(climate);
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder = QueryBuilder::new("INSERT INTO planet_terrains (planet_id, terrain)");
    query_builder.push_values(
        planets.iter().flat_map(|planet| {
            planet
                .terrains
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|terrain| (planet.id, terrain))
        }),
        |mut builder, (planet_id, terrain)| {
            builder
                .push_bind(i32::try_from(planet_id).unwrap())
                .push_bind(terrain);
        },
    );

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
    climates: Option<Vec<Climate>>,
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
    terrains: Option<Vec<Terrain>>,
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
    climates: Option<Vec<Climate>>,
    id: u32,
    name: String,
    surface_water: Option<f64>,
    diameter: Option<u32>,
    rotation_period: Option<u32>,
    terrains: Option<Vec<Terrain>>,
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

#[derive(Copy, Clone, Debug, Type)]
enum Climate {
    Arid,
    Temperate,
    Tropical,
    Frozen,
    Murky,
    Windy,
    Hot,
    ArtificialTemperate,
    Frigid,
    Humid,
    Moist,
    Polluted,
    Superheated,
    Subarctic,
    Arctic,
    Rocky,
}

impl FromStr for Climate {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "arid" => Ok(Self::Arid),
            "temperate" => Ok(Self::Temperate),
            "tropical" => Ok(Self::Tropical),
            "frozen" => Ok(Self::Frozen),
            "murky" => Ok(Self::Murky),
            "windy" => Ok(Self::Windy),
            "hot" => Ok(Self::Hot),
            "artificial temperate" => Ok(Self::ArtificialTemperate),
            "frigid" => Ok(Self::Frigid),
            "humid" => Ok(Self::Humid),
            "moist" => Ok(Self::Moist),
            "polluted" => Ok(Self::Polluted),
            "superheated" => Ok(Self::Superheated),
            "subartic" => Ok(Self::Subarctic),
            "artic" => Ok(Self::Arctic),
            "rocky" => Ok(Self::Rocky),
            _ => Err("Unknown climate"),
        }
    }
}

#[derive(Copy, Clone, Debug, Type)]
enum Terrain {
    Desert,
    Grasslands,
    Mountains,
    Jungle,
    Rainforests,
    Tundra,
    IceCaves,
    MountainRanges,
    Swamp,
    GasGiant,
    Forests,
    Lakes,
    GrassyHills,
    Cityscape,
    Ocean,
    Rock,
    Barren,
    Scrublands,
    Savanna,
    Canyons,
    Sinkholes,
    Volcanoes,
    LavaRivers,
    Caves,
    Rivers,
    AirlessAsteroid,
    Glaciers,
    IceCanyons,
    FungusForests,
    Fields,
    RockArches,
    Grass,
    Plains,
    Urban,
    Hills,
    Bogs,
    RockyIslands,
    Seas,
    Mesas,
    Islands,
    Reefs,
    RockyDeserts,
    Valleys,
    Ash,
    ToxicCloudsea,
    Plateaus,
    Verdant,
    RockyCanyons,
    AcidPools,
    Rocky,
    Vines,
    Cities,
    Cliffs,
}

impl FromStr for Terrain {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "desert" | "deserts" => Ok(Self::Desert),
            "grasslands" => Ok(Self::Grasslands),
            "mountains" | "mountain" => Ok(Self::Mountains),
            "jungle" | "jungles" => Ok(Self::Jungle),
            "rainforests" => Ok(Self::Rainforests),
            "tundra" => Ok(Self::Tundra),
            "ice caves" => Ok(Self::IceCaves),
            "mountain ranges" => Ok(Self::MountainRanges),
            "swamp" | "swamps" => Ok(Self::Swamp),
            "gas giant" => Ok(Self::GasGiant),
            "forests" => Ok(Self::Forests),
            "lakes" => Ok(Self::Lakes),
            "grassy hills" => Ok(Self::GrassyHills),
            "cityscape" => Ok(Self::Cityscape),
            "ocean" | "oceans" => Ok(Self::Ocean),
            "rock" => Ok(Self::Rock),
            "barren" => Ok(Self::Barren),
            "scrublands" => Ok(Self::Scrublands),
            "savanna" | "savannas" | "savannahs" => Ok(Self::Savanna),
            "canyons" => Ok(Self::Canyons),
            "sinkholes" => Ok(Self::Sinkholes),
            "volcanoes" => Ok(Self::Volcanoes),
            "lava rivers" => Ok(Self::LavaRivers),
            "caves" => Ok(Self::Caves),
            "rivers" => Ok(Self::Rivers),
            "airless asteroid" => Ok(Self::AirlessAsteroid),
            "glaciers" => Ok(Self::Glaciers),
            "ice canyons" => Ok(Self::IceCanyons),
            "fungus forests" => Ok(Self::FungusForests),
            "fields" => Ok(Self::Fields),
            "rock arches" => Ok(Self::RockArches),
            "grass" => Ok(Self::Grass),
            "plains" => Ok(Self::Plains),
            "urban" => Ok(Self::Urban),
            "hills" => Ok(Self::Hills),
            "bogs" => Ok(Self::Bogs),
            "rocky islands" => Ok(Self::RockyIslands),
            "seas" => Ok(Self::Seas),
            "mesas" => Ok(Self::Mesas),
            "islands" => Ok(Self::Islands),
            "reefs" => Ok(Self::Reefs),
            "rocky deserts" => Ok(Self::RockyDeserts),
            "valleys" => Ok(Self::Valleys),
            "ash" => Ok(Self::Ash),
            "toxic cloudsea" => Ok(Self::ToxicCloudsea),
            "plateaus" => Ok(Self::Plateaus),
            "verdant" => Ok(Self::Verdant),
            "rocky canyons" => Ok(Self::RockyCanyons),
            "acid pools" => Ok(Self::AcidPools),
            "rocky" => Ok(Self::Rocky),
            "vines" => Ok(Self::Vines),
            "cities" => Ok(Self::Cities),
            "cliffs" => Ok(Self::Cliffs),
            _ => Err("Unknown terrain"),
        }
    }
}

async fn seed_species(db_pool: &Pool<Postgres>) -> anyhow::Result<HashMap<u32, u32>> {
    let species = parse_json_file::<Vec<SpeciesNested>>("species").await?;

    let mut seen_person_ids: HashSet<u32> = _d();

    let people_species: HashMap<u32, u32> = species
        .iter()
        .flat_map(|species| {
            species
                .fields
                .people
                .iter()
                .map(|person_id| {
                    if seen_person_ids.contains(person_id) {
                        panic!("Already saw person ID {person_id}");
                    }
                    seen_person_ids.insert(*person_id);
                    (*person_id, species.id)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let species: Vec<Species> = species.into_iter().map(Into::into).collect();
    println!("species: {species:#?}");

    let mut query_builder = QueryBuilder::new("INSERT INTO species (id, edited, created, name, classification, designation, language, homeworld, average_lifespan, average_height)");
    query_builder.push_values(&species, |mut builder, species| {
        builder
            .push_bind(i32::try_from(species.id).unwrap())
            .push_bind(species.edited.to_sqlx())
            .push_bind(species.created.to_sqlx())
            .push_bind(species.name.clone())
            .push_bind(species.classification.clone())
            .push_bind(species.designation.clone())
            .push_bind(species.language.clone())
            .push_bind(
                species
                    .homeworld
                    .map(|homeworld| i32::try_from(homeworld).unwrap()),
            )
            .push_bind(
                species
                    .average_lifespan
                    .map(|average_lifespan| i32::try_from(average_lifespan).unwrap()),
            )
            .push_bind(species.average_height);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO species_skin_colors (species_id, skin_color)");
    query_builder.push_values(
        species.iter().flat_map(|species| {
            species
                .skin_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|skin_color| (species.id, skin_color))
        }),
        |mut builder, (species_id, skin_color)| {
            builder
                .push_bind(i32::try_from(species_id).unwrap())
                .push_bind(skin_color);
        },
    );

    let mut query_builder =
        QueryBuilder::new("INSERT INTO species_eye_colors (species_id, eye_color)");
    query_builder.push_values(
        species.iter().flat_map(|species| {
            species
                .eye_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|eye_color| (species.id, eye_color))
        }),
        |mut builder, (species_id, eye_color)| {
            builder
                .push_bind(i32::try_from(species_id).unwrap())
                .push_bind(eye_color);
        },
    );

    let mut query_builder =
        QueryBuilder::new("INSERT INTO species_hair_colors (species_id, hair_color)");
    query_builder.push_values(
        species.iter().flat_map(|species| {
            species
                .hair_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|hair_color| (species.id, hair_color))
        }),
        |mut builder, (species_id, hair_color)| {
            builder
                .push_bind(i32::try_from(species_id).unwrap())
                .push_bind(hair_color);
        },
    );

    Ok(people_species)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpeciesNested {
    fields: SpeciesNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpeciesNestedFields {
    edited: Timestamp,
    created: Timestamp,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    classification: Option<SpeciesClassification>,
    designation: SpeciesDesignation,
    #[serde(deserialize_with = "deserialize_comma_separated_or_unknown")]
    eye_colors: Option<Vec<EyeColor>>,
    people: Vec<u32>,
    #[serde(deserialize_with = "deserialize_comma_separated_or_unknown")]
    skin_colors: Option<Vec<SkinColor>>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    language: Option<String>,
    #[serde(deserialize_with = "deserialize_comma_separated_or_unknown")]
    hair_colors: Option<Vec<HairColor>>,
    homeworld: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    average_lifespan: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    average_height: Option<f64>,
}

#[derive(Debug)]
struct Species {
    id: u32,
    edited: Timestamp,
    created: Timestamp,
    name: String,
    classification: Option<SpeciesClassification>,
    designation: SpeciesDesignation,
    eye_colors: Option<Vec<EyeColor>>,
    skin_colors: Option<Vec<SkinColor>>,
    language: Option<String>,
    hair_colors: Option<Vec<HairColor>>,
    homeworld: Option<u32>,
    average_lifespan: Option<u32>,
    average_height: Option<f64>,
}

impl From<SpeciesNested> for Species {
    fn from(value: SpeciesNested) -> Self {
        Self {
            edited: value.fields.edited,
            created: value.fields.created,
            id: value.id,
            name: value.fields.name,
            classification: value.fields.classification,
            designation: value.fields.designation,
            eye_colors: value.fields.eye_colors,
            skin_colors: value.fields.skin_colors,
            language: value.fields.language,
            hair_colors: value.fields.hair_colors,
            homeworld: value.fields.homeworld,
            average_lifespan: value.fields.average_lifespan,
            average_height: value.fields.average_height,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
enum SpeciesClassification {
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

#[derive(Copy, Clone, Debug, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
enum SpeciesDesignation {
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

async fn seed_people(
    db_pool: &Pool<Postgres>,
    people_species: &HashMap<u32, u32>,
) -> anyhow::Result<()> {
    let people: Vec<Person> = parse_json_file::<Vec<PersonNested>>("people")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("people: {people:#?}");

    let mut query_builder = QueryBuilder::new("INSERT INTO people (id, edited, created, name, gender, height, mass, homeworld, birth_year)");
    query_builder.push_values(&people, |mut builder, person| {
        builder
            .push_bind(i32::try_from(person.id).unwrap())
            .push_bind(person.edited.to_sqlx())
            .push_bind(person.created.to_sqlx())
            .push_bind(person.name.clone())
            .push_bind(person.gender.clone())
            .push_bind(person.height.map(|height| i32::try_from(height).unwrap()))
            .push_bind(person.mass)
            .push_bind(i32::try_from(person.homeworld).unwrap())
            .push_bind(person.birth_year.clone());
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO person_skin_colors (person_id, skin_color)");
    query_builder.push_values(
        people.iter().flat_map(|person| {
            person
                .skin_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|skin_color| (person.id, skin_color))
        }),
        |mut builder, (person_id, skin_color)| {
            builder
                .push_bind(i32::try_from(person_id).unwrap())
                .push_bind(skin_color);
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO person_eye_colors (person_id, eye_color)");
    query_builder.push_values(
        people.iter().flat_map(|person| {
            person
                .eye_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|eye_color| (person.id, eye_color))
        }),
        |mut builder, (person_id, eye_color)| {
            builder
                .push_bind(i32::try_from(person_id).unwrap())
                .push_bind(eye_color);
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO person_hair_colors (person_id, hair_color)");
    query_builder.push_values(
        people.iter().flat_map(|person| {
            person
                .hair_colors
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|hair_color| (person.id, hair_color))
        }),
        |mut builder, (person_id, hair_color)| {
            builder
                .push_bind(i32::try_from(person_id).unwrap())
                .push_bind(hair_color);
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

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

#[derive(Copy, Clone, Debug, Deserialize, Type)]
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

#[derive(Copy, Clone, Debug, Deserialize, Type)]
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
    Magenta,
    Purple,
    Pink,
    PalePink,
    Peach,
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
            "magenta" => Ok(Self::Magenta),
            "purple" => Ok(Self::Purple),
            "pink" => Ok(Self::Pink),
            "pale pink" => Ok(Self::PalePink),
            "peach" => Ok(Self::Peach),
            _ => Err("Unknown skin color"),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Type)]
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

#[derive(Copy, Clone, Debug, Deserialize, Type)]
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
    Indigo,
    Silver,
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
            "indigo" => Ok(Self::Indigo),
            "silver" => Ok(Self::Silver),
            _ => Err("Unknown eye color"),
        }
    }
}

async fn seed_transports(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let transports: Vec<Transport> = parse_json_file::<Vec<TransportNested>>("transport")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("transports: {transports:#?}");

    let mut query_builder = QueryBuilder::new("INSERT INTO transports (id, edited, created, consumables, name, cargo_capacity, passengers, max_atmosphering_speed, crew_start, crew_end, length, model, cost_in_credits)");
    query_builder.push_values(&transports, |mut builder, transport| {
        builder
            .push_bind(i32::try_from(transport.id).unwrap())
            .push_bind(transport.edited.to_sqlx())
            .push_bind(transport.created.to_sqlx())
            .push_bind(transport.consumables.clone())
            .push_bind(transport.name.clone())
            .push_bind(transport.cargo_capacity)
            .push_bind(
                transport
                    .passengers
                    .map(|passengers| i32::try_from(passengers).unwrap()),
            )
            .push_bind(
                transport
                    .max_atmosphering_speed
                    .map(|max_atmosphering_speed| i32::try_from(max_atmosphering_speed).unwrap()),
            )
            .push_bind(
                transport
                    .crew
                    .as_ref()
                    .map(|crew| match crew {
                        SingleOrRange::Single(single) => *single,
                        SingleOrRange::Range(range) => *range.start(),
                    })
                    .map(|crew| i32::try_from(crew).unwrap()),
            )
            .push_bind(
                transport
                    .crew
                    .as_ref()
                    .and_then(|crew| match crew {
                        SingleOrRange::Single(_) => None,
                        SingleOrRange::Range(range) => Some(*range.end()),
                    })
                    .map(|crew| i32::try_from(crew).unwrap()),
            )
            .push_bind(transport.length)
            .push_bind(transport.model.clone())
            .push_bind(transport.cost_in_credits);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO transport_manufacturers (transport_id, manufacturer)");
    query_builder.push_values(
        transports.iter().flat_map(|transport| {
            transport
                .manufacturers
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|manufacturer| (transport.id, manufacturer))
        }),
        |mut builder, (transport_id, manufacturer)| {
            builder
                .push_bind(i32::try_from(transport_id).unwrap())
                .push_bind(manufacturer);
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let starships: Vec<Starship> = parse_json_file::<Vec<StarshipNested>>("starships")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("starships: {starships:#?}");

    let mut query_builder =
        QueryBuilder::new("INSERT INTO starships (id, mglt, starship_class, hyperdrive_rating)");
    query_builder.push_values(&starships, |mut builder, starship| {
        builder
            .push_bind(i32::try_from(starship.id).unwrap())
            .push_bind(starship.mglt.map(|mglt| i32::try_from(mglt).unwrap()))
            .push_bind(starship.starship_class)
            .push_bind(starship.hyperdrive_rating);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO starship_pilots (starship_id, person_id)");
    query_builder.push_values(
        starships.iter().flat_map(|starship| {
            starship
                .pilots
                .clone()
                .into_iter()
                .map(|pilot_id| (starship.id, pilot_id))
        }),
        |mut builder, (starship_id, pilot_id)| {
            builder
                .push_bind(i32::try_from(starship_id).unwrap())
                .push_bind(i32::try_from(pilot_id).unwrap());
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let vehicles: Vec<Vehicle> = parse_json_file::<Vec<VehicleNested>>("vehicles")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("vehicles: {vehicles:#?}");

    let mut query_builder = QueryBuilder::new("INSERT INTO vehicles (id, vehicle_class)");
    query_builder.push_values(&vehicles, |mut builder, vehicle| {
        builder
            .push_bind(i32::try_from(vehicle.id).unwrap())
            .push_bind(vehicle.vehicle_class);
    });

    let query = query_builder.build();
    query.execute(db_pool).await?;

    let mut query_builder = QueryBuilder::new("INSERT INTO vehicle_pilots (vehicle_id, person_id)");
    query_builder.push_values(
        vehicles.iter().flat_map(|vehicle| {
            vehicle
                .pilots
                .clone()
                .into_iter()
                .map(|pilot_id| (vehicle.id, pilot_id))
        }),
        |mut builder, (vehicle_id, pilot_id)| {
            builder
                .push_bind(i32::try_from(vehicle_id).unwrap())
                .push_bind(i32::try_from(pilot_id).unwrap());
        },
    );

    let query = query_builder.build();
    query.execute(db_pool).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TransportNested {
    fields: TransportNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TransportNestedFields {
    edited: Timestamp,
    created: Timestamp,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    consumables: Option<String>,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    cargo_capacity: Option<f64>,
    #[serde(deserialize_with = "deserialize_from_str_strip_commas_or_unknown")]
    passengers: Option<u32>,
    #[serde(deserialize_with = "deserialize_from_str_strip_km_or_unknown")]
    max_atmosphering_speed: Option<u32>,
    #[serde(deserialize_with = "deserialize_single_or_range_or_unknown")]
    crew: Option<SingleOrRange>,
    #[serde(deserialize_with = "deserialize_from_str_strip_commas_or_unknown")]
    length: Option<f64>,
    model: String,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    cost_in_credits: Option<f64>,
    #[serde(
        rename = "manufacturer",
        deserialize_with = "deserialize_comma_separated_not_inc_or_unknown"
    )]
    manufacturers: Option<Vec<Manufacturer>>,
}

#[derive(Debug)]
struct Transport {
    id: u32,
    edited: Timestamp,
    created: Timestamp,
    consumables: Option<String>,
    name: String,
    cargo_capacity: Option<f64>,
    passengers: Option<u32>,
    max_atmosphering_speed: Option<u32>,
    crew: Option<SingleOrRange>,
    length: Option<f64>,
    model: String,
    cost_in_credits: Option<f64>,
    manufacturers: Option<Vec<Manufacturer>>,
}

impl From<TransportNested> for Transport {
    fn from(value: TransportNested) -> Self {
        Self {
            edited: value.fields.edited,
            created: value.fields.created,
            id: value.id,
            consumables: value.fields.consumables,
            name: value.fields.name,
            cargo_capacity: value.fields.cargo_capacity,
            passengers: value.fields.passengers,
            max_atmosphering_speed: value.fields.max_atmosphering_speed,
            crew: value.fields.crew,
            length: value.fields.length,
            model: value.fields.model,
            cost_in_credits: value.fields.cost_in_credits,
            manufacturers: value.fields.manufacturers,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Type)]
enum Manufacturer {
    CorellianEngineeringCorporation,
    KuatDriveYards,
    CorelliaMiningCorporation,
    SienarFleetSystems,
    CyngusSpaceworks,
    IncomCorporation,
    SoroSuubCorporation,
    ImperialDepartmentOfMilitaryResearch,
    KoensayrManufacturing,
    FondorShipyards,
    GallofreeYardsInc,
    BespinMotors,
    KuatSystemsEngineering,
    UbrikkianIndustriesCustomVehicleDivision,
    UbrikkianIndustries,
    MonCalamariShipyards,
    AllianceUndergroundEngineering,
    SlaynAndKorpil,
    AratechRepulsorCompany,
    HoerschKesselDriveInc,
    HaorChallEngineering,
    BaktoidArmorWorkshop,
    OtohGungaBongamekenCooperative,
    TheedPalaceSpaceVesselEngineeringCorps,
    NubiaStarDrives,
    RepublicSienarSystems,
    Razalon,
    MobquetSwoopsAndSpeeders,
    DeslerGizhOutworldMobilityCorporation,
    NarglatchAirTechPrefabricatedKit,
    BotajefShipyards,
    RothanaHeavyEngineering,
    HupplaPasaTiscShipwrightsCollective,
    RendiliStarDrive,
    FreeDacVolunteersEngineeringCorps,
    ZGomotTernbuellGuppatCorporation,
    AllanteenSixShipyards,
    SubproCorporation,
    CollaDesigns,
    PhlacArphoccAutomataIndustries,
    GworiRevolutionaryIndustries,
    AppazannaEngineeringWorks,
    TechnoUnion,
    BaktoidFleetOrdnance,
    FeethanOttrawScalableAssemblies,
}

impl FromStr for Manufacturer {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Corellian Engineering Corporation" => Ok(Self::CorellianEngineeringCorporation),
            "Kuat Drive Yards" => Ok(Self::KuatDriveYards),
            "Corellia Mining Corporation" => Ok(Self::CorelliaMiningCorporation),
            "Sienar Fleet Systems" => Ok(Self::SienarFleetSystems),
            "Cyngus Spaceworks" | "Cygnus Spaceworks" => Ok(Self::CyngusSpaceworks),
            "Incom Corporation" | "Incom corporation" => Ok(Self::IncomCorporation),
            "SoroSuub Corporation" => Ok(Self::SoroSuubCorporation),
            "Imperial Department of Military Research" => {
                Ok(Self::ImperialDepartmentOfMilitaryResearch)
            }
            "Koensayr Manufacturing" => Ok(Self::KoensayrManufacturing),
            "Fondor Shipyards" => Ok(Self::FondorShipyards),
            "Gallofree Yards, Inc." => Ok(Self::GallofreeYardsInc),
            "Bespin Motors" => Ok(Self::BespinMotors),
            "Kuat Systems Engineering" => Ok(Self::KuatSystemsEngineering),
            "Ubrikkian Industries Custom Vehicle Division" => {
                Ok(Self::UbrikkianIndustriesCustomVehicleDivision)
            }
            "Ubrikkian Industries" => Ok(Self::UbrikkianIndustries),
            "Mon Calamari shipyards" => Ok(Self::MonCalamariShipyards),
            "Alliance Underground Engineering" => Ok(Self::AllianceUndergroundEngineering),
            "Slayn & Korpil" => Ok(Self::SlaynAndKorpil),
            "Aratech Repulsor Company" => Ok(Self::AratechRepulsorCompany),
            "Hoersch-Kessel Drive, Inc." | "Hoersch-Kessel Drive, Inc" => {
                Ok(Self::HoerschKesselDriveInc)
            }
            "Haor Chall Engineering" => Ok(Self::HaorChallEngineering),
            "Baktoid Armor Workshop" => Ok(Self::BaktoidArmorWorkshop),
            "Otoh Gunga Bongameken Cooperative" => Ok(Self::OtohGungaBongamekenCooperative),
            "Theed Palace Space Vessel Engineering Corps" => {
                Ok(Self::TheedPalaceSpaceVesselEngineeringCorps)
            }
            "Nubia Star Drives" | "Nubia Star Drives, Incorporated" => Ok(Self::NubiaStarDrives),
            "Republic Sienar Systems" => Ok(Self::RepublicSienarSystems),
            "Razalon" => Ok(Self::Razalon),
            "Mobquet Swoops and Speeders" => Ok(Self::MobquetSwoopsAndSpeeders),
            "Desler Gizh Outworld Mobility Corporation" => {
                Ok(Self::DeslerGizhOutworldMobilityCorporation)
            }
            "Narglatch AirTech prefabricated kit" => Ok(Self::NarglatchAirTechPrefabricatedKit),
            "Botajef Shipyards" => Ok(Self::BotajefShipyards),
            "Rothana Heavy Engineering" => Ok(Self::RothanaHeavyEngineering),
            "Huppla Pasa Tisc Shipwrights Collective" => {
                Ok(Self::HupplaPasaTiscShipwrightsCollective)
            }
            "Rendili StarDrive" => Ok(Self::RendiliStarDrive),
            "Free Dac Volunteers Engineering corps." => Ok(Self::FreeDacVolunteersEngineeringCorps),
            "Z-Gomot Ternbuell Guppat Corporation" => Ok(Self::ZGomotTernbuellGuppatCorporation),
            "Allanteen Six shipyards" => Ok(Self::AllanteenSixShipyards),
            "Subpro Corporation" => Ok(Self::SubproCorporation),
            "Colla Designs" => Ok(Self::CollaDesigns),
            "Phlac-Arphocc Automata Industries" => Ok(Self::PhlacArphoccAutomataIndustries),
            "Gwori Revolutionary Industries" => Ok(Self::GworiRevolutionaryIndustries),
            "Appazanna Engineering Works" => Ok(Self::AppazannaEngineeringWorks),
            "Techno Union" => Ok(Self::TechnoUnion),
            "Baktoid Fleet Ordnance" => Ok(Self::BaktoidFleetOrdnance),
            "Feethan Ottraw Scalable Assemblies" => Ok(Self::FeethanOttrawScalableAssemblies),
            _ => Err("Unknown manufacturer"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StarshipNested {
    fields: StarshipNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StarshipNestedFields {
    pilots: Vec<u32>,
    #[serde(alias = "MGLT", deserialize_with = "deserialize_from_str_or_unknown")]
    mglt: Option<u32>,
    starship_class: StarshipClass,
    #[serde(deserialize_with = "deserialize_from_str_or_unknown")]
    hyperdrive_rating: Option<f64>,
}

#[derive(Debug)]
struct Starship {
    id: u32,
    pilots: Vec<u32>,
    mglt: Option<u32>,
    starship_class: StarshipClass,
    hyperdrive_rating: Option<f64>,
}

impl From<StarshipNested> for Starship {
    fn from(value: StarshipNested) -> Self {
        Self {
            id: value.id,
            pilots: value.fields.pilots,
            mglt: value.fields.mglt,
            starship_class: value.fields.starship_class,
            hyperdrive_rating: value.fields.hyperdrive_rating,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Type)]
enum StarshipClass {
    #[serde(alias = "corvette")]
    Corvette,
    #[serde(alias = "Star Destroyer")]
    #[serde(alias = "star destroyer")]
    StarDestroyer,
    #[serde(alias = "landing craft")]
    LandingCraft,
    #[serde(alias = "Deep Space Mobile Battlestation")]
    DeepSpaceMobileBattlestation,
    #[serde(alias = "Light freighter")]
    LightFreighter,
    #[serde(alias = "assault starfighter")]
    #[serde(alias = "Assault Starfighter")]
    AssaultStarfighter,
    #[serde(alias = "starfighter")]
    Starfighter,
    #[serde(alias = "Star dreadnought")]
    StarDreadnought,
    #[serde(alias = "Medium transport")]
    MediumTransport,
    #[serde(alias = "Patrol craft")]
    PatrolCraft,
    #[serde(alias = "Armed government transport")]
    ArmedGovernmentTransport,
    #[serde(alias = "Escort ship")]
    EscortShip,
    #[serde(alias = "Star Cruiser")]
    StarCruiser,
    #[serde(alias = "Space cruiser")]
    SpaceCruiser,
    #[serde(alias = "Droid control ship")]
    DroidControlShip,
    #[serde(alias = "yacht")]
    Yacht,
    #[serde(alias = "Space Transport")]
    SpaceTransport,
    #[serde(alias = "Diplomatic barge")]
    DiplomaticBarge,
    #[serde(alias = "freighter")]
    Freighter,
    #[serde(alias = "assault ship")]
    AssaultShip,
    #[serde(alias = "capital ship")]
    CapitalShip,
    #[serde(alias = "transport")]
    Transport,
    #[serde(alias = "cruiser")]
    Cruiser,
}

#[derive(Debug)]
enum SingleOrRange {
    Single(u32),
    Range(RangeInclusive<u32>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VehicleNested {
    fields: VehicleNestedFields,
    #[serde(rename = "pk")]
    id: u32,
    #[serde(rename = "model")]
    _model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VehicleNestedFields {
    pilots: Vec<u32>,
    vehicle_class: VehicleClass,
}

#[derive(Debug)]
struct Vehicle {
    id: u32,
    pilots: Vec<u32>,
    vehicle_class: VehicleClass,
}

impl From<VehicleNested> for Vehicle {
    fn from(value: VehicleNested) -> Self {
        Self {
            id: value.id,
            pilots: value.fields.pilots,
            vehicle_class: value.fields.vehicle_class,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Type)]
enum VehicleClass {
    #[serde(alias = "wheeled")]
    Wheeled,
    #[serde(alias = "repulsorcraft")]
    Repulsorcraft,
    #[serde(alias = "starfighter")]
    Starfighter,
    #[serde(alias = "airspeeder")]
    #[serde(alias = "air speeder")]
    Airspeeder,
    #[serde(alias = "space/planetary bomber")]
    SpacePlanetaryBomber,
    #[serde(alias = "assault walker")]
    AssaultWalker,
    #[serde(alias = "walker")]
    Walker,
    #[serde(alias = "sail barge")]
    SailBarge,
    #[serde(alias = "repulsorcraft cargo skiff")]
    RepulsorcraftCargoSkiff,
    #[serde(alias = "speeder")]
    Speeder,
    #[serde(alias = "landing craft")]
    LandingCraft,
    #[serde(alias = "submarine")]
    Submarine,
    #[serde(alias = "gunship")]
    Gunship,
    #[serde(alias = "transport")]
    Transport,
    #[serde(alias = "wheeled walker")]
    WheeledWalker,
    #[serde(alias = "fire suppression ship")]
    FireSuppressionShip,
    #[serde(alias = "droid starfighter")]
    DroidStarfighter,
    #[serde(alias = "droid tank")]
    DroidTank,
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
        "unknown" | "n/a" | "none" | "indefinite" => None,
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
        "unknown" | "n/a" | "none" | "indefinite" => None,
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

fn deserialize_comma_separated_not_inc_or_unknown<'de, TTarget, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<Vec<TTarget>>, TDeserializer::Error>
where
    TTarget: FromStr,
    TTarget::Err: Display,
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(match &*str {
        "unknown" | "n/a" | "none" | "indefinite" => None,
        str => Some(
            fancy_regex!(r#", (?!Inc\b|Incorporated)"#)
                .split(str)
                .map(Result::unwrap)
                .map(|chunk| {
                    match fancy_regex!(r#",(?! Inc\b| Incorporated)"#)
                        .is_match(chunk)
                        .unwrap()
                    {
                        false => TTarget::from_str(chunk).map_err(|_| ".from_str() failed"),
                        true => Err("Unexpected format"),
                    }
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

fn deserialize_single_or_range<'de, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<SingleOrRange, TDeserializer::Error>
where
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?.replace(",", "");
    Ok(
        if let Some(captures) = regex!(r#"^(\d+)-(\d+)$"#).captures(&str) {
            SingleOrRange::Range(RangeInclusive::new(
                captures[1].parse::<u32>().unwrap(),
                captures[2].parse::<u32>().unwrap(),
            ))
        } else {
            let single = str.parse::<u32>().map_err(de::Error::custom)?;
            SingleOrRange::Single(single)
        },
    )
}

fn deserialize_single_or_range_or_unknown<'de, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<SingleOrRange>, TDeserializer::Error>
where
    TDeserializer: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?.replace(",", "");
    Ok(match &*str {
        "unknown" | "n/a" | "none" => None,
        str => Some(
            if let Some(captures) = regex!(r#"^(\d+)-(\d+)$"#).captures(&str) {
                SingleOrRange::Range(RangeInclusive::new(
                    captures[1].parse::<u32>().unwrap(),
                    captures[2].parse::<u32>().unwrap(),
                ))
            } else {
                let single = str.parse::<u32>().map_err(de::Error::custom)?;
                SingleOrRange::Single(single)
            },
        ),
    })
}

fn deserialize_from_str_strip_km_or_unknown<'de, TTarget, TDeserializer>(
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
        str => Some(TTarget::from_str(&str.replace("km", "")).map_err(de::Error::custom)?),
    })
}
