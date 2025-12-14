CREATE TABLE planets (
  -- id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  id INTEGER PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  name TEXT NOT NULL,
  surface_water FLOAT,
  diameter INTEGER,
  rotation_period INTEGER,
  gravity TEXT,
  orbital_period INTEGER,
  population FLOAT
);

CREATE TYPE climate AS ENUM (
  'Arid',
  'Temperate',
  'Tropical',
  'Frozen',
  'Murky',
  'Windy',
  'Hot',
  'ArtificialTemperate',
  'Frigid',
  'Humid',
  'Moist',
  'Polluted',
  'Superheated',
  'Subarctic',
  'Arctic',
  'Rocky'
);

CREATE TABLE planet_climates (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  planet_id integer REFERENCES planets (id) NOT NULL,
  climate climate NOT NULL
);

CREATE TYPE terrain AS ENUM (
  'Desert',
  'Grasslands',
  'Mountains',
  'Jungle',
  'Rainforests',
  'Tundra',
  'IceCaves',
  'MountainRanges',
  'Swamp',
  'GasGiant',
  'Forests',
  'Lakes',
  'GrassyHills',
  'Cityscape',
  'Ocean',
  'Rock',
  'Barren',
  'Scrublands',
  'Savanna',
  'Canyons',
  'Sinkholes',
  'Volcanoes',
  'LavaRivers',
  'Caves',
  'Rivers',
  'AirlessAsteroid',
  'Glaciers',
  'IceCanyons',
  'FungusForests',
  'Fields',
  'RockArches',
  'Grass',
  'Plains',
  'Urban',
  'Hills',
  'Bogs',
  'RockyIslands',
  'Seas',
  'Mesas',
  'Islands',
  'Reefs',
  'RockyDeserts',
  'Valleys',
  'Ash',
  'ToxicCloudsea',
  'Plateaus',
  'Verdant',
  'RockyCanyons',
  'AcidPools',
  'Rocky',
  'Vines',
  'Cities',
  'Cliffs'
);

CREATE TABLE planet_terrains (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  planet_id integer REFERENCES planets (id) NOT NULL,
  terrain terrain NOT NULL
);

CREATE TYPE speciesclassification AS ENUM (
  'Mammal',
  'Artificial',
  'Sentient',
  'Gastropod',
  'Reptile',
  'Amphibian',
  'Insectoid',
  'Reptilian'
);

CREATE TYPE speciesdesignation AS ENUM (
  'Sentient',
  'Reptilian'
);

CREATE TYPE skincolor AS ENUM (
  'Caucasian',
  'Black',
  'Asian',
  'Hispanic',
  'Grey',
  'Fair',
  'Gold',
  'White',
  'Blue',
  'Light',
  'Red',
  'Green',
  'GreenTan',
  'Brown',
  'Pale',
  'Metal',
  'Dark',
  'BrownMottle',
  'MottledGreen',
  'Orange',
  'Yellow',
  'Tan',
  'Silver',
  'Magenta',
  'Purple',
  'Pink',
  'PalePink',
  'Peach'
);

CREATE TYPE eyecolor AS ENUM (
  'Brown',
  'Blue',
  'Green',
  'Hazel',
  'Grey',
  'Amber',
  'Yellow',
  'Golden',
  'Red',
  'Black',
  'BlueGray',
  'Orange',
  'Pink',
  'Gold',
  'White',
  'Indigo',
  'Silver'
);

CREATE TYPE haircolor AS ENUM (
  'Blonde',
  'Brown',
  'Black',
  'Red',
  'Grey',
  'Auburn',
  'White'
);

CREATE TYPE language AS ENUM (
  'GalacticBasic',
  'Shyriiwook',
  'Huttese',
  'Dosh',
  'MonCalamarian',
  'Ewokese',
  'Sullutese',
  'Neimoidia',
  'GunganBasic',
  'Toydarian',
  'Dugese',
  'TwiLeki',
  'Aleena',
  'Vulpterish',
  'Xextese',
  'Tundan',
  'Cerean',
  'Nautila',
  'Zabraki',
  'Iktotchese',
  'Quermian',
  'KelDor',
  'Chagria',
  'Geonosian',
  'Mirialan',
  'Clawdite',
  'Besalisk',
  'Kaminoan',
  'Skakoan',
  'Muun',
  'Togruti',
  'Kaleesh',
  'Utapese'
);

CREATE TABLE species (
  id INTEGER PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  name TEXT NOT NULL,
  classification speciesclassification,
  designation speciesdesignation NOT NULL,
  language language,
  homeworld_id integer REFERENCES planets (id),
  average_lifespan INTEGER,
  average_height FLOAT
);

CREATE TABLE species_skin_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  skin_color skincolor NOT NULL
);

CREATE TABLE species_eye_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  eye_color eyecolor NOT NULL
);

CREATE TABLE species_hair_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  hair_color haircolor NOT NULL
);

CREATE TYPE gender AS ENUM ('Male', 'Female', 'Hermaphrodite');

CREATE TABLE people (
  id INTEGER PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  name TEXT NOT NULL,
  gender gender,
  height INTEGER,
  mass FLOAT,
  homeworld integer REFERENCES planets (id) NOT NULL,
  birth_year TEXT,
  species_id integer REFERENCES species (id)
);

CREATE TABLE person_skin_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  skin_color skincolor NOT NULL
);

CREATE TABLE person_eye_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  eye_color eyecolor NOT NULL
);

CREATE TABLE person_hair_colors (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  hair_color haircolor NOT NULL
);

CREATE TABLE transports (
  id INTEGER PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  consumables TEXT,
  name TEXT NOT NULL,
  cargo_capacity FLOAT,
  passengers INTEGER,
  max_atmosphering_speed INTEGER,
  crew_start INTEGER,
  crew_end INTEGER,
  length FLOAT,
  model TEXT NOT NULL,
  cost_in_credits FLOAT
);

CREATE TYPE manufacturer AS ENUM (
  'CorellianEngineeringCorporation',
  'KuatDriveYards',
  'CorelliaMiningCorporation',
  'SienarFleetSystems',
  'CyngusSpaceworks',
  'IncomCorporation',
  'SoroSuubCorporation',
  'ImperialDepartmentOfMilitaryResearch',
  'KoensayrManufacturing',
  'FondorShipyards',
  'GallofreeYardsInc',
  'BespinMotors',
  'KuatSystemsEngineering',
  'UbrikkianIndustriesCustomVehicleDivision',
  'UbrikkianIndustries',
  'MonCalamariShipyards',
  'AllianceUndergroundEngineering',
  'SlaynAndKorpil',
  'AratechRepulsorCompany',
  'HoerschKesselDriveInc',
  'HaorChallEngineering',
  'BaktoidArmorWorkshop',
  'OtohGungaBongamekenCooperative',
  'TheedPalaceSpaceVesselEngineeringCorps',
  'NubiaStarDrives',
  'RepublicSienarSystems',
  'Razalon',
  'MobquetSwoopsAndSpeeders',
  'DeslerGizhOutworldMobilityCorporation',
  'NarglatchAirTechPrefabricatedKit',
  'BotajefShipyards',
  'RothanaHeavyEngineering',
  'HupplaPasaTiscShipwrightsCollective',
  'RendiliStarDrive',
  'FreeDacVolunteersEngineeringCorps',
  'ZGomotTernbuellGuppatCorporation',
  'AllanteenSixShipyards',
  'SubproCorporation',
  'CollaDesigns',
  'PhlacArphoccAutomataIndustries',
  'GworiRevolutionaryIndustries',
  'AppazannaEngineeringWorks',
  'TechnoUnion',
  'BaktoidFleetOrdnance',
  'FeethanOttrawScalableAssemblies'
);

CREATE TABLE transport_manufacturers (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  transport_id integer REFERENCES transports (id) NOT NULL,
  manufacturer manufacturer NOT NULL
);

CREATE TYPE starshipclass AS ENUM(
  'Corvette',
  'StarDestroyer',
  'LandingCraft',
  'DeepSpaceMobileBattlestation',
  'LightFreighter',
  'AssaultStarfighter',
  'Starfighter',
  'StarDreadnought',
  'MediumTransport',
  'PatrolCraft',
  'ArmedGovernmentTransport',
  'EscortShip',
  'StarCruiser',
  'SpaceCruiser',
  'DroidControlShip',
  'Yacht',
  'SpaceTransport',
  'DiplomaticBarge',
  'Freighter',
  'AssaultShip',
  'CapitalShip',
  'Transport',
  'Cruiser'
);

CREATE TABLE starships (
  id INTEGER PRIMARY KEY NOT NULL,
  mglt INTEGER,
  starship_class starshipclass NOT NULL,
  hyperdrive_rating FLOAT,
  transport_id integer REFERENCES transports (id) NOT NULL
);

CREATE TABLE starship_pilots (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  starship_id integer REFERENCES starships (id) NOT NULL,
  person_id integer REFERENCES people (id) NOT NULL
);

CREATE TYPE vehicleclass AS ENUM(
  'Wheeled',
  'Repulsorcraft',
  'Starfighter',
  'Airspeeder',
  'SpacePlanetaryBomber',
  'AssaultWalker',
  'Walker',
  'SailBarge',
  'RepulsorcraftCargoSkiff',
  'Speeder',
  'LandingCraft',
  'Submarine',
  'Gunship',
  'Transport',
  'WheeledWalker',
  'FireSuppressionShip',
  'DroidStarfighter',
  'DroidTank'
);

CREATE TABLE vehicles (
  id INTEGER PRIMARY KEY NOT NULL,
  vehicle_class vehicleclass NOT NULL,
  transport_id integer REFERENCES transports (id) NOT NULL
);

CREATE TABLE vehicle_pilots (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  vehicle_id integer REFERENCES vehicles (id) NOT NULL,
  person_id integer REFERENCES people (id) NOT NULL
);

CREATE TYPE producerordirector AS ENUM(
  'GaryKurtz',
  'RickMcCallum',
  'GeorgeLucas',
  'IrvinKershner',
  'HowardGKazanjian',
  'RichardMarquand'
);

CREATE TABLE films (
  id INTEGER PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  title TEXT NOT NULL,
  episode_id INTEGER NOT NULL,
  director producerordirector NOT NULL,
  release_date DATE NOT NULL,
  opening_crawl TEXT NOT NULL
);

CREATE TABLE film_starships (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  starship_id integer REFERENCES starships (id) NOT NULL
);

CREATE TABLE film_vehicles (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  vehicle_id integer REFERENCES vehicles (id) NOT NULL
);

CREATE TABLE film_planets (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  planet_id integer REFERENCES planets (id) NOT NULL
);

CREATE TABLE film_producers (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  producer producerordirector NOT NULL
);

CREATE TABLE film_characters (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  person_id integer REFERENCES people (id) NOT NULL
);

CREATE TABLE film_species (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  film_id integer REFERENCES films (id) NOT NULL,
  species_id integer REFERENCES species (id) NOT NULL
);
