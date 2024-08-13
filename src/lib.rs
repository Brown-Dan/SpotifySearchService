use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::models::{Album, Artist, Stream, Track, Upload, UploadableStream};
use crate::schema::{albums, artists, streams, tracks, upload};

pub mod schema;
pub mod models;

pub fn establish_database_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_upload_by_id(id: uuid::Uuid) -> Option<Upload> {
    let connection = &mut establish_database_connection();

    return upload::table
        .filter(upload::upload_id.eq(id))
        .select(upload::all_columns)
        .first::<Upload>(connection).ok();
}

pub fn insert_upload(upload: Upload) {
    let connection = &mut establish_database_connection();
    diesel::insert_into(upload::table)
        .values(upload)
        .returning(Upload::as_returning())
        .get_result(connection)
        .expect("Error saving new artist");
}

pub fn insert_stream_data(stream: &UploadableStream) {
    let connection = &mut establish_database_connection();

    let artist = artists::table
        .filter(artists::uri.eq(&stream.track.artist.uri))
        .select(artists::all_columns)
        .first::<Artist>(connection);

    if artist.is_err() {
        insert_artist(&stream.track.artist, connection);
    }

    let album = albums::table
        .filter(albums::uri.eq(&stream.track.album.uri))
        .select(albums::all_columns)
        .first::<Album>(connection);

    if album.is_err() {
        insert_album(&stream.track.album, connection);
    }

    let track = tracks::table
        .filter(tracks::uri.eq(&stream.track.uri))
        .select(tracks::all_columns)
        .first::<Track>(connection);

    if track.is_err() {
        let insertable_track = Track {
            uri: stream.track.uri.clone(),
            name: stream.track.name.clone(),
            album_uri: Some(stream.track.album.uri.clone()),
            artist_uri: Some(stream.track.artist.uri.clone()),
        };
        insert_track(&insertable_track, connection);
    }

    let insertable_stream = Stream {
        username: Some(stream.username.clone()),
        time_streamed: Some(stream.time_streamed),
        stream_length_ms: Some(stream.stream_length_ms.clone()),
        track_uri: Some(stream.track.uri.clone()),
    };
    insert_stream(&insertable_stream, connection);
}

pub fn insert_artist(artist: &Artist, connection: &mut PgConnection) {
    diesel::insert_into(artists::table)
        .values(artist)
        .returning(Artist::as_returning())
        .get_result(connection)
        .expect("Error saving new artist");
}

pub fn insert_track(track: &Track, connection: &mut PgConnection) {
    diesel::insert_into(tracks::table)
        .values(track)
        .returning(Track::as_returning())
        .get_result(connection)
        .expect("Error saving new track");
}

pub fn insert_album(album: &Album, connection: &mut PgConnection) {
    diesel::insert_into(albums::table)
        .values(album)
        .returning(Album::as_returning())
        .get_result(connection)
        .expect("Error saving new album");
}

pub fn insert_stream(stream: &Stream, connection: &mut PgConnection) {
    diesel::insert_into(streams::table)
        .values(stream)
        .returning(Stream::as_returning())
        .get_result(connection)
        .expect("Error saving new stream");
}