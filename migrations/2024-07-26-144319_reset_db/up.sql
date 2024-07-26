-- Your SQL goes here
-- Your SQL goes here
CREATE OR REPLACE FUNCTION truncate_tables(username IN VARCHAR) RETURNS int AS $$
DECLARE
    statements CURSOR FOR
        SELECT tablename FROM pg_tables
        WHERE tableowner = username AND schemaname = 'public';
BEGIN
    FOR stmt IN statements LOOP
        IF stmt.tablename != '__diesel_schema_migrations' then 
            EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.tablename) || ' CASCADE;';
        END IF;
    END LOOP;
    RETURN 0;
END;
$$ LANGUAGE plpgsql;

