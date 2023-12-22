-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_fts_first_category;
DROP INDEX IF EXISTS idx_fts_second_category;
DROP INDEX IF EXISTS idx_fts_manufacturer;
DROP INDEX IF EXISTS idx_fts_library_type;
DROP INDEX IF EXISTS idx_fts_description;

DROP TRIGGER IF EXISTS update_updated_at_trigger ON components;
DROP FUNCTION IF EXISTS update_updated_at;

DROP TABLE IF EXISTS components;


