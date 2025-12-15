use sauvignon::{schema, Schema};

use shared::{
    Climate, EyeColor, Gender, HairColor, Language, ProducerOrDirector, SkinColor,
    SpeciesClassification, SpeciesDesignation, StarshipClass, Terrain, VehicleClass,
};

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
                    id => id_column()
                    orbitalPeriod => optional_int_column()
                    population => optional_float_column()
                    residents => has_many(
                        type => Person
                        foreign_key => homeworld_id
                    )
                    rotationPeriod => optional_int_column()
                    surfaceWater => optional_float_column()
                    climates => has_many(
                        type => Climate
                        through => planet_climates
                    )
                    terrains => has_many(
                        type => Terrain
                        through => planet_terrains
                    )
                ]
            }
            Film => {
                fields => [
                    title => string_column()
                    characters => has_many(
                        type => Person
                        through => film_characters
                    )
                    created => timestamp_column()
                    edited => timestamp_column()
                    director => enum_column(
                        type => ProducerOrDirector
                    )
                    episodeId => int_column()
                    id => id_column()
                    openingCrawl => string_column()
                    planets => has_many(
                        type => Planet
                        through => film_planets
                    )
                    // producers => has_many(
                    //     type => ProducerOrDirector
                    //     through => film_producers
                    // )
                    species => has_many(
                        type => Species
                        through => film_species
                    )
                    starships => has_many(
                        type => Starship
                        through => film_starships
                    )
                    vehicles => has_many(
                        type => Vehicle
                        through => film_vehicles
                    )
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
                    eyeColors => has_many(
                        type => EyeColor
                        through => species_eye_colors
                    )
                    hairColors => has_many(
                        type => HairColor
                        through => species_hair_colors
                    )
                    skinColors => has_many(
                        type => SkinColor
                        through => species_skin_colors
                    )
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
                    birthYear => optional_string_column()
                    created => timestamp_column()
                    edited => timestamp_column()
                    eyeColors => has_many(
                        type => EyeColor
                        through => person_eye_colors
                    )
                    films => has_many(
                        type => Film
                        through => film_characters
                    )
                    gender => optional_enum_column(
                        type => Gender
                    )
                    hairColors => has_many(
                        type => HairColor
                        through => person_hair_colors
                    )
                    height => optional_int_column()
                    homeworld => belongs_to(
                        type => Planet
                    )
                    id => id_column()
                    mass => optional_float_column()
                    skinColors => has_many(
                        type => SkinColor
                        through => person_skin_colors
                    )
                    species => belongs_to(
                        type => Species
                        optional => true
                    )
                    starships => has_many(
                        type => Starship
                        through => starship_pilots
                    )
                    vehicles => has_many(
                        type => Vehicle
                        through => vehicle_pilots
                    )
                ]
            }
            Starship => {
                fields => [
                    pilots => has_many(
                        type => Person
                        through => starship_pilots
                    )
                    starshipClass => enum_column(
                        type => StarshipClass
                    )
                    mglt => optional_int_column()
                    id => id_column()
                    hyperdriveRating => optional_float_column()
                ]
            }
            Vehicle => {
                fields => [
                    vehicleClass => enum_column(
                        type => VehicleClass
                    )
                    id => id_column()
                    pilots => has_many(
                        type => Person
                        through => vehicle_pilots
                    )
                    created => timestamp_column(
                        via_nested => {
                            table_name => transports
                            id_column_name => id
                        }
                    )
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
            allFilms => {
                type => [Film!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
            allPeople => {
                type => [Person!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
            allStarships => {
                type => [Starship!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
            allVehicles => {
                type => [Vehicle!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
        ]
        enums => [
            SpeciesClassification,
            SpeciesDesignation,
            Language,
            Climate,
            Terrain,
            EyeColor,
            HairColor,
            SkinColor,
            ProducerOrDirector,
            StarshipClass,
            VehicleClass,
            Gender,
        ]
    }
}
