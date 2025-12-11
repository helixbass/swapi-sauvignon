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
