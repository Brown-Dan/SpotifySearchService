CREATE TABLE albums
(
    uri        TEXT PRIMARY KEY,
    name       TEXT,
    image_urls TEXT[]
);

CREATE TABLE artists
(
    uri  TEXT PRIMARY KEY,
    name TEXT
);

CREATE TABLE tracks
(
    uri        TEXT PRIMARY KEY,
    name       TEXT,
    album_uri  TEXT REFERENCES albums (uri),
    artist_uri TEXT REFERENCES artists (uri)
);

CREATE TABLE streams
(
    id               SERIAL PRIMARY KEY,
    username         TEXT,
    time_streamed    TIMESTAMP,
    stream_length_ms NUMERIC,
    track_uri        TEXT REFERENCES tracks (uri)
);
