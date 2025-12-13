use sauvignon::{schema, Response, Schema};
use sqlx::{Pool, Postgres};

pub fn get_schema() -> Schema {
    schema! {
        types => [
            Planet => {
                fields => [
                    name => string_column()
                    films => has_many(
                        // TODO: update when `type` has a default
                        // to remove `type` here
                        type => Film
                        through => film_planets
                    )
                ]
            }
            Film => {
                fields => [
                    title => string_column()
                ]
            }
            Species => {
                fields => [
                    name => string_column()
                    // TODO: should be able to be just
                    // has_many() (with no args)?
                    people => has_many(
                        type => Person
                        foreign_key => species_id
                    )
                ]
            }
            Person => {
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
            allSpecies => {
                type => [Species!]!
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
