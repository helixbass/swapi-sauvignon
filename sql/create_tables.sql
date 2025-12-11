CREATE TABLE planets (
  -- id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  id INT PRIMARY KEY NOT NULL,
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
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
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
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
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

CREATE TABLE species (
  id INT PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  name TEXT NOT NULL,
  classification speciesclassification,
  designation speciesdesignation NOT NULL,
  homeworld integer REFERENCES planets (id),
  average_lifespan INTEGER,
  average_height FLOAT
);

CREATE TABLE species_skin_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  skin_color skincolor NOT NULL
);

CREATE TABLE species_eye_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  eye_color eyecolor NOT NULL
);

CREATE TABLE species_hair_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  species_id integer REFERENCES species (id) NOT NULL,
  hair_color haircolor NOT NULL
);

CREATE TYPE gender AS ENUM ('Male', 'Female', 'Hermaphrodite');

CREATE TABLE people (
  id INT PRIMARY KEY NOT NULL,
  edited TIMESTAMPTZ NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  name TEXT NOT NULL,
  gender gender,
  height INTEGER,
  mass FLOAT,
  homeworld integer REFERENCES planets (id) NOT NULL,
  birth_year TEXT
);

CREATE TABLE person_skin_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  skin_color skincolor NOT NULL
);

CREATE TABLE person_eye_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  eye_color eyecolor NOT NULL
);

CREATE TABLE person_hair_colors (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  person_id integer REFERENCES people (id) NOT NULL,
  hair_color haircolor NOT NULL
);
