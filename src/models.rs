use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamData {
    #[serde(alias = "uploadId")]
    upload_id: String,
    username: String,
    streams: Vec<UploadableStream>,
    #[serde(alias = "firstStream")]
    first_stream: chrono::NaiveDateTime,
    #[serde(alias = "lastStream")]
    last_stream: chrono::NaiveDateTime,
    #[serde(alias = "numberOfStreams")]
    number_of_streams: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct UploadableStream {
    username: String,
    #[serde(alias = "timeStreamed")]
    time_streamed: chrono::NaiveDateTime,
    #[serde(alias = "streamLengthMs")]
    stream_length_ms: bigdecimal::BigDecimal,
    track: UploadableTrack,
}

#[derive(Deserialize, Serialize, Debug)]
struct UploadableTrack {
    uri: String,
    name: Option<String>,
    album: Album,
    artist: Artist,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct Album {
    pub uri: String,
    pub name: Option<String>,
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
    pub id: i32,
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