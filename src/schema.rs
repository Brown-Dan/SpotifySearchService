// @generated automatically by Diesel CLI.

diesel::table! {
    albums (uri) {
        uri -> Text,
        name -> Nullable<Text>,
        image_urls -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    artists (uri) {
        uri -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    streams (id) {
        id -> Int4,
        username -> Nullable<Text>,
        time_streamed -> Nullable<Timestamp>,
        stream_length_ms -> Nullable<Numeric>,
        track_uri -> Nullable<Text>,
    }
}

diesel::table! {
    tracks (uri) {
        uri -> Text,
        name -> Nullable<Text>,
        album_uri -> Nullable<Text>,
        artist_uri -> Nullable<Text>,
    }
}

diesel::table! {
    uploads (upload_id) {
        upload_id -> Uuid,
        username -> Nullable<Text>,
        first_stream -> Nullable<Timestamp>,
        last_stream -> Nullable<Timestamp>,
        number_of_streams -> Nullable<Int4>,
        #[max_length = 36]
        status -> Nullable<Varchar>,
        message -> Nullable<Text>,
    }
}

diesel::joinable!(streams -> tracks (track_uri));
diesel::joinable!(tracks -> albums (album_uri));
diesel::joinable!(tracks -> artists (artist_uri));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    streams,
    tracks,
    uploads,
);
