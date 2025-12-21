use sauvignon::{
    enum_optional_string_massager, enum_string_massager, schema, CarverOrPopulator,
    ExternalDependencyValues, IntCarver, InternalDependencyValues,
    OptionalUnionOrInterfaceTypePopulator, Populator, PopulatorInterface, PostgresColumnMassager,
    PostgresDatabase, Schema,
};
use smol_str::SmolStr;
use sqlx::{Pool, Postgres};

use shared::{
    Climate, EyeColor, Gender, HairColor, Language, Manufacturer, ProducerOrDirector, SkinColor,
    SpeciesClassification, SpeciesDesignation, StarshipClass, Terrain, VehicleClass,
};

#[derive(Default)]
struct SingleOrRangeTypePopulator {}

impl OptionalUnionOrInterfaceTypePopulator for SingleOrRangeTypePopulator {
    fn populate(
        &self,
        _external_dependencies: &ExternalDependencyValues,
        internal_dependencies: &InternalDependencyValues,
    ) -> Option<SmolStr> {
        internal_dependencies
            .get("crew_start")
            .unwrap()
            .maybe_non_optional()?;
        Some(
            match internal_dependencies
                .get("crew_end")
                .unwrap()
                .maybe_non_optional()
            {
                None => "SingleOrRangeSingle".into(),
                Some(_) => "SingleOrRangeRange".into(),
            },
        )
    }
}

#[derive(Default)]
struct SingleOrRangePopulator {}

impl PopulatorInterface for SingleOrRangePopulator {
    fn populate(
        &self,
        _external_dependencies: &ExternalDependencyValues,
        internal_dependencies: &InternalDependencyValues,
    ) -> ExternalDependencyValues {
        let mut ret = ExternalDependencyValues::default();
        let Some(crew_start) = internal_dependencies
            .get("crew_start")
            .unwrap()
            .maybe_non_optional()
        else {
            return ret;
        };
        match internal_dependencies
            .get("crew_end")
            .unwrap()
            .maybe_non_optional()
        {
            Some(crew_end) => {
                ret.insert("start".into(), crew_start.clone()).unwrap();
                ret.insert("end".into(), crew_end.clone()).unwrap();
            }
            None => {
                ret.insert("value".into(), crew_start.clone()).unwrap();
            }
        }
        ret
    }
}

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
                    releaseDate => date_column()
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
                    created => timestamp_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    edited => timestamp_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    cargoCapacity => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    consumables => optional_string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    costInCredits => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    crew => {
                        type => SingleOrRange,
                        internal_dependencies => [
                            crew_start => optional_int_column(
                                via_nested => {
                                    table_name => transports
                                    foreign_key => id
                                }
                            )
                            crew_end => optional_int_column(
                                via_nested => {
                                    table_name => transports
                                    foreign_key => id
                                }
                            )
                        ]
                        populator => custom {
                            CarverOrPopulator::OptionalUnionOrInterfaceTypePopulator(
                                Box::new(SingleOrRangeTypePopulator::default()),
                                Populator::Dyn(Box::new(SingleOrRangePopulator::default())),
                            )
                        }
                    }
                    films => has_many(
                        type => Film
                        through => film_starships
                    )
                    length => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    manufacturers => has_many(
                        type => Manufacturer
                        through => {
                            table_name => transport_manufacturers
                            self_foreign_key => transport_id
                        }
                        // TODO: this is only unnecessary b/c
                        // we happen to use vehicles.id = transports.id
                        // as the nested join "same column"/foreign key,
                        // so should in theory support "normal" via_nested
                        // + through has_many combination?
                        // via_nested => {
                        //     table_name => transports
                        //     foreign_key => id
                        // }
                    )
                    maxAtmospheringSpeed => optional_int_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    model => string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    name => string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    passengers => optional_int_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
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
                            foreign_key => id
                        }
                    )
                    edited => timestamp_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    cargoCapacity => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    consumables => optional_string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    costInCredits => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    crew => {
                        type => SingleOrRange,
                        internal_dependencies => [
                            crew_start => optional_int_column(
                                via_nested => {
                                    table_name => transports
                                    foreign_key => id
                                }
                            )
                            crew_end => optional_int_column(
                                via_nested => {
                                    table_name => transports
                                    foreign_key => id
                                }
                            )
                        ]
                        populator => custom {
                            CarverOrPopulator::OptionalUnionOrInterfaceTypePopulator(
                                Box::new(SingleOrRangeTypePopulator::default()),
                                Populator::Dyn(Box::new(SingleOrRangePopulator::default())),
                            )
                        }
                    }
                    films => has_many(
                        type => Film
                        through => film_vehicles
                    )
                    length => optional_float_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    manufacturers => has_many(
                        type => Manufacturer
                        through => {
                            table_name => transport_manufacturers
                            self_foreign_key => transport_id
                        }
                    )
                    maxAtmospheringSpeed => optional_int_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    model => string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    name => string_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                    passengers => optional_int_column(
                        via_nested => {
                            table_name => transports
                            foreign_key => id
                        }
                    )
                ]
            }
            SingleOrRangeSingle => {
                fields => [
                    value => {
                        type => Int!
                        carver => custom {
                            CarverOrPopulator::Carver(Box::new(IntCarver::new("value".into())))
                        }
                    }
                ]
            }
            SingleOrRangeRange => {
                fields => [
                    start => {
                        type => Int!
                        carver => custom {
                            CarverOrPopulator::Carver(Box::new(IntCarver::new("start".into())))
                        }
                    }
                    end => {
                        type => Int!
                        carver => custom {
                            CarverOrPopulator::Carver(Box::new(IntCarver::new("end".into())))
                        }
                    }
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
            planet => {
                type => Planet!
                params => [
                    id => Id!
                ]
            }
            film => {
                type => Film!
                params => [
                    id => Id!
                ]
            }
            person => {
                type => Person!
                params => [
                    id => Id!
                ]
            }
            species => {
                type => Species!
                params => [
                    id => Id!
                ]
            }
            starship => {
                type => Starship!
                params => [
                    id => Id!
                ]
            }
            vehicle => {
                type => Vehicle!
                params => [
                    id => Id!
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
            Manufacturer,
        ]
        unions => [
            SingleOrRange => [SingleOrRangeSingle, SingleOrRangeRange]
        ]
    }
}

pub fn get_database(db_pool: Pool<Postgres>) -> PostgresDatabase {
    PostgresDatabase::new(
        db_pool,
        [
            ("planet_climates", "climate", enum_string_massager!(Climate)),
            ("planet_terrains", "terrain", enum_string_massager!(Terrain)),
            (
                "species",
                "classification",
                enum_optional_string_massager!(SpeciesClassification),
            ),
            (
                "species",
                "designation",
                enum_string_massager!(SpeciesDesignation),
            ),
            (
                "species",
                "language",
                enum_optional_string_massager!(Language),
            ),
            (
                "species_skin_colors",
                "skin_color",
                enum_string_massager!(SkinColor),
            ),
            (
                "species_eye_colors",
                "eye_color",
                enum_string_massager!(EyeColor),
            ),
            (
                "species_hair_colors",
                "hair_color",
                enum_string_massager!(HairColor),
            ),
            ("people", "gender", enum_optional_string_massager!(Gender)),
            (
                "person_skin_colors",
                "skin_color",
                enum_string_massager!(SkinColor),
            ),
            (
                "person_eye_colors",
                "eye_color",
                enum_string_massager!(EyeColor),
            ),
            (
                "person_hair_colors",
                "hair_color",
                enum_string_massager!(HairColor),
            ),
            (
                "transport_manufacturers",
                "manufacturer",
                enum_string_massager!(Manufacturer),
            ),
            (
                "starships",
                "starship_class",
                enum_string_massager!(StarshipClass),
            ),
            (
                "vehicles",
                "vehicle_class",
                enum_string_massager!(VehicleClass),
            ),
            (
                "films",
                "director",
                enum_string_massager!(ProducerOrDirector),
            ),
            (
                "film_producers",
                "producer",
                enum_string_massager!(ProducerOrDirector),
            ),
        ]
        .into_iter()
        .map(|(table_name, column_name, massager)| {
            PostgresColumnMassager::new(table_name.into(), column_name.into(), massager)
        })
        .collect(),
    )
}
