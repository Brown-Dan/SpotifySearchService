use std::fmt;

use actix_web::HttpResponse;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum UploadStatus {
    Processing,
    Failed,
    Completed,
}

impl fmt::Display for UploadStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorKey {
    EntityNotFound,
    InternalServerError,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub key: ErrorKey,
    pub message: String,
}

pub fn ok(body: String) -> HttpResponse {
    return HttpResponse::Ok().body(body);
}

pub fn entity_not_found(id: &str) -> HttpResponse {
    let body = serde_json::to_string(
        &Error {
            key: ErrorKey::EntityNotFound,
            message: format!("Entity not found for id '{}'", id),
        }
    ).unwrap();
    return HttpResponse::NotFound().body(body);
}

pub fn internal_server_error(message: &str) -> HttpResponse {
    let body = serde_json::to_string(
        &Error {
            key: ErrorKey::InternalServerError,
            message: String::from(message),
        }
    ).unwrap();
    return HttpResponse::InternalServerError().body(body);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamData {
    #[serde(alias = "uploadId")]
    pub upload_id: String,
    pub username: String,
    pub streams: Vec<UploadableStream>,
    #[serde(alias = "firstStream")]
    pub first_stream: chrono::NaiveDateTime,
    #[serde(alias = "lastStream")]
    pub last_stream: chrono::NaiveDateTime,
    #[serde(alias = "numberOfStreams")]
    pub number_of_streams: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadableStream {
    pub(crate) username: String,
    #[serde(alias = "timeStreamed")]
    pub(crate) time_streamed: chrono::NaiveDateTime,
    #[serde(alias = "streamLengthMs")]
    pub(crate) stream_length_ms: bigdecimal::BigDecimal,
    pub track: UploadableTrack,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadableTrack {
    pub uri: String,
    pub name: Option<String>,
    pub album: Album,
    pub artist: Artist,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Album {
    pub uri: String,
    pub name: Option<String>,
    #[serde(alias = "images")]
    pub image_urls: Option<Vec<Option<String>>>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Artist {
    pub uri: String,
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::streams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Stream {
    pub username: Option<String>,
    pub time_streamed: Option<chrono::NaiveDateTime>,
    pub stream_length_ms: Option<bigdecimal::BigDecimal>,
    pub track_uri: Option<String>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Track {
    pub uri: String,
    pub name: Option<String>,
    pub album_uri: Option<String>,
    pub artist_uri: Option<String>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::uploads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Upload {
    pub upload_id: uuid::Uuid,
    pub username: Option<String>,
    pub first_stream: Option<chrono::NaiveDateTime>,
    pub last_stream: Option<chrono::NaiveDateTime>,
    pub number_of_streams: Option<i32>,
    pub status: Option<String>,
    pub message: Option<String>,
}
