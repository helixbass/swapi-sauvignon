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
