use sauvignon::json_from_response;
use serde_json_path::{JsonPath, NodeList};
use swapi_sauvignon::get_schema;

use shared::get_db_pool;

async fn request_test(request: &str, expected: impl FnOnce(&serde_json::Value)) {
    let db_pool = get_db_pool().await.unwrap();
    let schema = get_schema();
    let response = schema.request(request, &db_pool).await;
    let json = json_from_response(&response);
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    expected(&json);
}

#[tokio::test]
async fn test_all_planets() {
    request_test(
        r#"
            {
              allPlanets {
                name
                films {
                  title
                }
                created
                edited
                diameter
                gravity
                id
                orbitalPeriod
                population
                residents {
                  name
                }
                rotationPeriod
                surfaceWater
                climates
                terrains
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allPlanets.*", response).len(), 60);
            assert_eq!(
                _q("$.data.allPlanets[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Tatooine"
            );
            assert_eq!(_q("$.data.allPlanets[0].films.*", response).len(), 5);
            assert_eq!(
                _q("$.data.allPlanets[0].films[0].title", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "A New Hope"
            );
            assert_eq!(
                _q("$.data.allPlanets[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-09T13:50:49.641Z"
            );
            assert_eq!(
                _q("$.data.allPlanets[0].edited", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-20T20:58:18.411Z"
            );
            assert_eq!(
                _q("$.data.allPlanets[0].diameter", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_u64()
                    .unwrap(),
                10465
            );
            assert_eq!(
                _q("$.data.allPlanets[37].diameter", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allPlanets[0].gravity", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "1 standard"
            );
            assert_eq!(
                _q("$.data.allPlanets[18].gravity", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allPlanets[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "1"
            );
            assert_eq!(
                _q("$.data.allPlanets[0].orbitalPeriod", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_u64()
                    .unwrap(),
                304
            );
            assert_eq!(
                _q("$.data.allPlanets[19].orbitalPeriod", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allPlanets[0].population", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                200000.0
            );
            assert_eq!(
                _q("$.data.allPlanets[3].population", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(_q("$.data.allPlanets[36].residents.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPlanets[36].residents[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Bib Fortuna"
            );
            assert_eq!(
                _q("$.data.allPlanets[0].rotationPeriod", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_i64()
                    .unwrap(),
                23
            );
            assert_eq!(
                _q("$.data.allPlanets[19].rotationPeriod", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allPlanets[0].surfaceWater", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                1.0
            );
            assert_eq!(
                _q("$.data.allPlanets[8].surfaceWater", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(_q("$.data.allPlanets[0].climates.*", response).len(), 1);
            assert_eq!(
                _q("$.data.allPlanets[0].climates[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "ARID"
            );
            assert_eq!(_q("$.data.allPlanets[2].climates.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPlanets[2].climates[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "TEMPERATE"
            );
            assert_eq!(_q("$.data.allPlanets[27].climates.*", response).len(), 0);
            assert_eq!(_q("$.data.allPlanets[0].terrains.*", response).len(), 1);
            assert_eq!(
                _q("$.data.allPlanets[0].terrains[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "DESERT"
            );
            assert_eq!(_q("$.data.allPlanets[1].terrains.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPlanets[1].terrains[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "GRASSLANDS"
            );
            assert_eq!(_q("$.data.allPlanets[27].terrains.*", response).len(), 0);
        },
    )
    .await;
}

#[tokio::test]
async fn test_all_species() {
    request_test(
        r#"
            {
              allSpecies {
                name
                averageHeight
                averageLifespan
                classification
                created
                designation
                people {
                  name
                }
                films {
                  title
                }
                homeworld {
                  name
                }
                id
                language
                edited
                eyeColors
                hairColors
                skinColors
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allSpecies.*", response).len(), 37);
            assert_eq!(
                _q("$.data.allSpecies[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Human"
            );
            assert_eq!(
                _q("$.data.allSpecies[0].averageHeight", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                180.0
            );
            assert_eq!(
                _q("$.data.allSpecies[0].averageLifespan", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_u64()
                    .unwrap(),
                120
            );
            assert_eq!(
                _q("$.data.allSpecies[0].classification", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "MAMMAL"
            );
            assert_eq!(
                _q("$.data.allSpecies[10].classification", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allSpecies[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-10T13:52:11.567Z"
            );
            assert_eq!(
                _q("$.data.allSpecies[0].designation", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "SENTIENT"
            );
            assert_eq!(
                _q("$.data.allSpecies[0].edited", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-20T21:36:42.136Z"
            );
            assert_eq!(_q("$.data.allSpecies[0].people.*", response).len(), 35);
            assert_eq!(
                _q("$.data.allSpecies[0].people[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Luke Skywalker"
            );
            assert_eq!(_q("$.data.allSpecies[4].films.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allSpecies[4].films[0].title", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "A New Hope"
            );
            assert_eq!(
                _q("$.data.allSpecies[0].homeworld.name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Coruscant"
            );
            assert_eq!(
                _q("$.data.allSpecies[1].homeworld", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allSpecies[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "1"
            );
            assert_eq!(
                _q("$.data.allSpecies[0].language", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "GALACTIC_BASIC"
            );
            assert_eq!(
                _q("$.data.allSpecies[22].language", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(_q("$.data.allSpecies[0].eyeColors.*", response).len(), 6);
            assert_eq!(
                _q("$.data.allSpecies[0].eyeColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "BROWN"
            );
            assert_eq!(_q("$.data.allSpecies[1].eyeColors.*", response).len(), 0);
            assert_eq!(_q("$.data.allSpecies[0].hairColors.*", response).len(), 4);
            assert_eq!(
                _q("$.data.allSpecies[0].hairColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "BLONDE"
            );
            assert_eq!(_q("$.data.allSpecies[1].hairColors.*", response).len(), 0);
            assert_eq!(_q("$.data.allSpecies[0].skinColors.*", response).len(), 4);
            assert_eq!(
                _q("$.data.allSpecies[0].skinColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "CAUCASIAN"
            );
            assert_eq!(_q("$.data.allSpecies[1].skinColors.*", response).len(), 0);
        },
    )
    .await;
}

#[tokio::test]
async fn test_all_films() {
    request_test(
        r#"
            {
              allFilms {
                title
                characters {
                  name
                }
                created
                edited
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allFilms.*", response).len(), 6);
            assert_eq!(
                _q("$.data.allFilms[0].title", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "A New Hope"
            );
            assert_eq!(_q("$.data.allFilms[0].characters.*", response).len(), 18);
            assert_eq!(
                _q("$.data.allFilms[0].characters[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Luke Skywalker"
            );
            assert_eq!(
                _q("$.data.allFilms[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-10T14:23:31.88Z"
            );
            assert_eq!(
                _q("$.data.allFilms[0].edited", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-20T19:49:45.256Z"
            );
        },
    )
    .await;
}

fn _q<'a>(query: &str, response: &'a serde_json::Value) -> NodeList<'a> {
    let path = JsonPath::parse(query).unwrap();
    path.query(response)
}
