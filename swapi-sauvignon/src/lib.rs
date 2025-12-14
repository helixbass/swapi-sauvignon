use sauvignon::{schema, Schema};

use shared::{Language, SpeciesClassification, SpeciesDesignation};

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
                    created => timestamp_column()
                    edited => timestamp_column()
                    diameter => optional_int_column()
                    gravity => optional_string_column()
                ]
            }
            Film => {
                fields => [
                    title => string_column()
                ]
            }
            Species => {
                fields => [
                    averageHeight => optional_float_column()
                    averageLifespan => optional_int_column()
                    classification => optional_enum_column(
                        type => SpeciesClassification
                    )
                    created => timestamp_column()
                    designation => enum_column(
                        type => SpeciesDesignation
                    )
                    edited => timestamp_column()
                    name => string_column()
                    // TODO: should be able to be just
                    // has_many() (with no args)?
                    people => has_many(
                        type => Person
                        foreign_key => species_id
                    )
                    films => has_many(
                        // TODO: update when `type` has a default
                        // to remove `type` here
                        type => Film
                        through => film_species
                    )
                    homeworld => belongs_to(
                        optional => true
                        type => Planet
                    )
                    id => id_column()
                    language => optional_enum_column(
                        type => Language
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
        enums => [
            SpeciesClassification,
            SpeciesDesignation,
            Language,
        ]
    }
}
