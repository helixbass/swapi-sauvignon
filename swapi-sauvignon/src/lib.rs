use sauvignon::{schema, Response, Schema};
use sqlx::{Pool, Postgres};

pub fn get_schema() -> Schema {
    schema! {
        types => [
            Planet => {
                fields => [
                    name => string_column()
                    // films => has_many(
                    //     through => film_planets
                    // )
                ]
            }
            Film => {
                fields => [
                    name => string_column()
                ]
            }
        ]
        query => [
            allPlanets => {
                type => [Planet!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
        ]
    }
}

pub async fn request(document_str: &str, schema: &Schema, db_pool: &Pool<Postgres>) -> Response {
    schema.request(document_str, db_pool).await
}
