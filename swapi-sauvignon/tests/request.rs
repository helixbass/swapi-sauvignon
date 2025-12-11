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
async fn test_object_field() {
    request_test(
        r#"
            {
              allPlanets {
                name
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.allPlanets.*", response).len(), 60);
        },
    )
    .await;
}

fn _q<'a>(query: &str, response: &'a serde_json::Value) -> NodeList<'a> {
    let path = JsonPath::parse(query).unwrap();
    path.query(response)
}
