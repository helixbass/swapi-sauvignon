use sauvignon::json_from_response;
use swapi_sauvignon::get_schema;

use shared::get_db_pool;

async fn request_test(request: &str, expected: &str) {
    let db_pool = get_db_pool().await.unwrap();
    let schema = get_schema();
    let response = schema.request(request, &db_pool).await;
    let json = json_from_response(&response);
    assert_eq!(pretty_print_json(&json), pretty_print_json(expected));
}

fn pretty_print_json(json: &str) -> String {
    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    serde_json::to_string_pretty(&parsed).unwrap()
}

#[tokio::test]
async fn test_object_field() {
    request_test(
        r#"
            query {
              actorKatie {
                name
              }
            }
        "#,
        r#"
            {
              "data": {
                "actorKatie": {
                  "name": "Katie Cassidy"
                }
              }
            }
        "#,
    )
    .await;
}
