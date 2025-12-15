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
                director
                episodeId
                id
                openingCrawl
                planets {
                  name
                }
                # producers
                species {
                  name
                }
                starships {
                  starshipClass
                }
                vehicles {
                  vehicleClass
                }
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
            assert_eq!(
                _q("$.data.allFilms[0].director", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "GEORGE_LUCAS"
            );
            assert_eq!(
                _q("$.data.allFilms[0].episodeId", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_i64()
                    .unwrap(),
                4
            );
            assert_eq!(
                _q("$.data.allFilms[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "1"
            );
            assert_eq!(
                &_q("$.data.allFilms[0].openingCrawl", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap()[..5],
                "It is"
            );
            assert_eq!(_q("$.data.allFilms[0].planets.*", response).len(), 3);
            assert_eq!(
                _q("$.data.allFilms[0].planets[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Tatooine"
            );
            // assert_eq!(_q("$.data.allFilms[0].producers.*", response).len(), 2);
            // assert_eq!(
            //     _q("$.data.allFilms[0].producers[0]", response)
            //         .exactly_one()
            //         .unwrap()
            //         .as_str()
            //         .unwrap(),
            //     "GARY_KURTZ"
            // );
            assert_eq!(_q("$.data.allFilms[0].species.*", response).len(), 5);
            assert_eq!(
                _q("$.data.allFilms[0].species[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Human"
            );
            assert_eq!(_q("$.data.allFilms[0].starships.*", response).len(), 8);
            assert_eq!(
                _q("$.data.allFilms[0].starships[0].starshipClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "CORVETTE"
            );
            assert_eq!(_q("$.data.allFilms[0].vehicles.*", response).len(), 4);
            assert_eq!(
                _q("$.data.allFilms[0].vehicles[0].vehicleClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "WHEELED"
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_all_people() {
    request_test(
        r#"
            {
              allPeople {
                name
                birthYear
                created
                edited
                eyeColors
                films {
                  title
                }
                gender
                hairColors
                height
                homeworld {
                  name
                }
                id
                mass
                skinColors
                species {
                  name
                }
                starships {
                  starshipClass
                }
                vehicles {
                  vehicleClass
                }
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allPeople.*", response).len(), 82);
            assert_eq!(
                _q("$.data.allPeople[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Luke Skywalker"
            );
            assert_eq!(
                _q("$.data.allPeople[0].birthYear", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "19BBY"
            );
            assert_eq!(
                _q("$.data.allPeople[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-09T13:50:51.644Z"
            );
            assert_eq!(
                _q("$.data.allPeople[0].edited", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-20T21:17:56.891Z"
            );
            assert_eq!(_q("$.data.allPeople[0].eyeColors.*", response).len(), 1);
            assert_eq!(
                _q("$.data.allPeople[0].eyeColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "BLUE"
            );
            assert_eq!(_q("$.data.allPeople[73].eyeColors.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPeople[73].eyeColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "RED"
            );
            assert_eq!(_q("$.data.allPeople[0].films.*", response).len(), 4);
            assert_eq!(
                _q("$.data.allPeople[0].films[0].title", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "A New Hope"
            );
            assert_eq!(
                _q("$.data.allPeople[0].gender", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "MALE"
            );
            assert_eq!(
                _q("$.data.allPeople[1].gender", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(_q("$.data.allPeople[0].hairColors.*", response).len(), 1);
            assert_eq!(
                _q("$.data.allPeople[0].hairColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "BLONDE"
            );
            assert_eq!(_q("$.data.allPeople[5].hairColors.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPeople[5].hairColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "BROWN"
            );
            assert_eq!(
                _q("$.data.allPeople[0].height", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_i64()
                    .unwrap(),
                172
            );
            assert_eq!(
                _q("$.data.allPeople[27].height", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allPeople[0].homeworld.name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Tatooine"
            );
            assert_eq!(
                _q("$.data.allPeople[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "1"
            );
            assert_eq!(
                _q("$.data.allPeople[0].mass", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                77.0
            );
            assert_eq!(
                _q("$.data.allPeople[11].mass", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(_q("$.data.allPeople[0].skinColors.*", response).len(), 1);
            assert_eq!(
                _q("$.data.allPeople[0].skinColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "FAIR"
            );
            assert_eq!(_q("$.data.allPeople[2].skinColors.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPeople[2].skinColors[0]", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "WHITE"
            );
            assert_eq!(
                _q("$.data.allPeople[0].species.name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Human"
            );
            assert_eq!(_q("$.data.allPeople[12].starships.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPeople[12].starships[0].starshipClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "LIGHT_FREIGHTER"
            );
            assert_eq!(_q("$.data.allPeople[0].vehicles.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allPeople[0].vehicles[0].vehicleClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "AIRSPEEDER"
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_all_starships() {
    request_test(
        r#"
            {
              allStarships {
                starshipClass
                pilots {
                  name
                }
                mglt
                id
                hyperdriveRating
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allStarships.*", response).len(), 36);
            assert_eq!(
                _q("$.data.allStarships[0].starshipClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "CORVETTE"
            );
            assert_eq!(_q("$.data.allStarships[4].pilots.*", response).len(), 4);
            assert_eq!(
                _q("$.data.allStarships[4].pilots[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Chewbacca"
            );
            assert_eq!(
                _q("$.data.allStarships[0].mglt", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_i64()
                    .unwrap(),
                60
            );
            assert_eq!(
                _q("$.data.allStarships[16].mglt", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allStarships[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2"
            );
            assert_eq!(
                _q("$.data.allStarships[0].hyperdriveRating", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                2.0
            );
            assert_eq!(
                _q("$.data.allStarships[22].hyperdriveRating", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_all_vehicles() {
    request_test(
        r#"
            {
              allVehicles {
                vehicleClass
                id
                pilots {
                  name
                }
                created
                edited
                cargoCapacity
                consumables
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allVehicles.*", response).len(), 39);
            assert_eq!(
                _q("$.data.allVehicles[0].vehicleClass", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "WHEELED"
            );
            assert_eq!(
                _q("$.data.allVehicles[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "4"
            );
            assert_eq!(_q("$.data.allVehicles[4].pilots.*", response).len(), 2);
            assert_eq!(
                _q("$.data.allVehicles[4].pilots[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Luke Skywalker"
            );
            assert_eq!(
                _q("$.data.allVehicles[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-10T15:36:25.724Z"
            );
            assert_eq!(
                _q("$.data.allVehicles[0].edited", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-20T21:30:21.661Z"
            );
            assert_eq!(
                _q("$.data.allVehicles[0].cargoCapacity", response)
                    .exactly_one()
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_f64()
                    .unwrap(),
                50000.0
            );
            assert_eq!(
                _q("$.data.allVehicles[15].cargoCapacity", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
            assert_eq!(
                _q("$.data.allVehicles[0].consumables", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2 months"
            );
            assert_eq!(
                _q("$.data.allVehicles[2].consumables", response)
                    .exactly_one()
                    .unwrap()
                    .as_null()
                    .unwrap(),
                ()
            );
        },
    )
    .await;
}

fn _q<'a>(query: &str, response: &'a serde_json::Value) -> NodeList<'a> {
    let path = JsonPath::parse(query).unwrap();
    path.query(response)
}
