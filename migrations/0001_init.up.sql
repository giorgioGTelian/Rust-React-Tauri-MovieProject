CREATE TABLE movies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    original_title TEXT,
    year INTEGER,
    file_path TEXT UNIQUE NOT NULL,
    tmdb_id INTEGER,
    poster_path TEXT,
    backdrop_path TEXT,
    overview TEXT,
    runtime INTEGER,
    genres TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tv_shows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    original_title TEXT,
    first_air_year INTEGER,
    tmdb_id INTEGER,
    poster_path TEXT,
    backdrop_path TEXT,
    overview TEXT,
    number_of_seasons INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE episodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    show_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    episode_number INTEGER NOT NULL,
    title TEXT,
    file_path TEXT UNIQUE NOT NULL,
    overview TEXT,
    FOREIGN KEY(show_id) REFERENCES tv_shows(id),
    UNIQUE(show_id, season_number, episode_number)
);