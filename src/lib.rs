use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

use crate::models::{Album, Artist, Stream, Track, Upload, UploadableStream};
use crate::schema::{albums, artists, streams, tracks, uploads};

pub mod schema;
pub mod models;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}


pub fn get_upload_by_id(id: uuid::Uuid, connection: &mut PgConnection) -> Option<Upload> {
    return uploads::table
        .filter(uploads::upload_id.eq(id))
        .select(uploads::all_columns)
        .first::<Upload>(connection).ok();
}

pub fn insert_upload(upload: Upload, connection: &mut PgConnection) {
    diesel::insert_into(uploads::table)
        .values(upload)
        .returning(Upload::as_returning())
        .get_result(connection)
        .expect("Error saving new artist");
}

pub fn insert_stream_data(streams: &Vec<UploadableStream>, connection: &mut PgConnection) {
    streams.iter().for_each(|stream| { insert_uploadable_stream(stream, connection) })
}

fn insert_uploadable_stream(stream: &UploadableStream, connection: &mut PgConnection) {
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