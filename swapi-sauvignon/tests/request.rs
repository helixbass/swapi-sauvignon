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
                people {
                  name
                }
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
                _q("$.data.allSpecies[0].created", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2014-12-10T13:52:11.567Z"
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
        },
    )
    .await;
}

fn _q<'a>(query: &str, response: &'a serde_json::Value) -> NodeList<'a> {
    let path = JsonPath::parse(query).unwrap();
    path.query(response)
}
