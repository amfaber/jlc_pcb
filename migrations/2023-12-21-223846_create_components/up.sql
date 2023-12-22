CREATE TABLE components(
    lcsc_part TEXT PRIMARY KEY,
    first_category TEXT,
    second_category TEXT,
    mfr_part TEXT,
    solder_joint TEXT,
    manufacturer TEXT,
    library_type TEXT,
    description TEXT,
    datasheet TEXT,
    price TEXT,
    stock INT,
    package TEXT,
    api_last_key TEXT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_updated_at_trigger
BEFORE UPDATE ON components
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE INDEX idx_fts_first_category ON components USING gin (to_tsvector('english', first_category));
CREATE INDEX idx_fts_second_category ON components USING gin (to_tsvector('english', second_category));
CREATE INDEX idx_fts_manufacturer ON components USING gin (to_tsvector('english', manufacturer));
CREATE INDEX idx_fts_library_type ON components USING gin (to_tsvector('english', library_type));
CREATE INDEX idx_fts_description ON components USING gin (to_tsvector('english', description));

