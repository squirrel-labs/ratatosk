CREATE TABLE game_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(25) NOT NULL,
    redirect_uri TEXT NOT NULL,
    capacity INTEGER,
    color CHAR(6) NOT NULL DEFAULT '424242',
    icon CHAR(6) NOT NULL DEFAULT ''
)
