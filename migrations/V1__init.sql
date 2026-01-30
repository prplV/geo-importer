CREATE TABLE IF NOT EXISTS geos (
    id serial PRIMARY KEY,
    latitude double precision NOT NULL,
    longitude double precision NOT NULL,
    date date NOT NULL,
    year integer GENERATED ALWAYS AS (EXTRACT(YEAR FROM date)) STORED,
    month integer GENERATED ALWAYS AS (EXTRACT(MONTH FROM date)) STORED,
    day integer GENERATED ALWAYS AS (EXTRACT(DAY FROM date)) STORED,
    data jsonb NOT NULL DEFAULT '{}',

    CONSTRAINT valid_latitude CHECK (latitude >= -90 AND latitude <= 90),
    CONSTRAINT valid_longitude CHECK (longitude >= -180 AND longitude <= 180)
);


CREATE INDEX IF NOT EXISTS idx_geos_date ON geos (date);
CREATE INDEX IF NOT EXISTS idx_geos_coords ON geos (latitude, longitude);
CREATE INDEX IF NOT EXISTS idx_geos_year_month ON geos (year, month);
CREATE INDEX IF NOT EXISTS idx_geos_jsonb ON geos USING gin (data);
