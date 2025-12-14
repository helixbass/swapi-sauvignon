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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum Climate {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum Terrain {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "kebab-case")]
pub enum EyeColor {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Type, VariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "snake_case")]
pub enum HairColor {
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
