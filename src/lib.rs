use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::models::{Album, Artist, Stream, Track};
use crate::schema::{albums, artists, streams, tracks};

pub mod schema;
pub mod models;

pub fn establish_database_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_artist(artist: &Artist) {
    let connection = &mut establish_database_connection();
    diesel::insert_into(artists::table)
        .values(artist)
        .returning(Artist::as_returning())
        .get_result(connection)
        .expect("Error saving new artist");
}

pub fn insert_track(track: &Track) {
    let connection = &mut establish_database_connection();
    diesel::insert_into(tracks::table)
        .values(track)
        .returning(Track::as_returning())
        .get_result(connection)
        .expect("Error saving new track");
}

pub fn insert_album(album: &Album) {
    let connection = &mut establish_database_connection();
    diesel::insert_into(albums::table)
        .values(album)
        .returning(Album::as_returning())
        .get_result(connection)
        .expect("Error saving new album");
}

pub fn insert_stream(stream: &Stream) {
    let connection = &mut establish_database_connection();
    diesel::insert_into(streams::table)
        .values(stream)
        .returning(Stream::as_returning())
        .get_result(connection)
        .expect("Error saving new stream");
}