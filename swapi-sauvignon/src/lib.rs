use sauvignon::{schema, Schema};

pub fn get_schema() -> Schema {
    schema! {
        types => [
            Planet => {
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
