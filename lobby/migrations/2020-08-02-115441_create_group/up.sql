CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(25) NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    game_type_id INTEGER NOT NULL REFERENCES game_types (id)
)
