use jiff::Timestamp;
use serde::Deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

pub async fn seed() -> anyhow::Result<()> {
    unimplemented!()
}

#[serde_as]
#[derive(Deserialize)]
struct Planet {
    edited: Timestamp,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    climates: Vec<String>,
}
