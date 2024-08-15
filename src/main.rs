use std::time::Duration;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;
use diesel::{PgConnection, r2d2};

use MusicInsightsSearchService::{get_connection_pool, get_upload_by_id, insert_stream_data, insert_upload};
use MusicInsightsSearchService::models::{entity_not_found, internal_server_error, ok, StreamData, Upload, UploadStatus};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: DbPool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(20 * 1024 * 1024))
            .service(root)
            .service(upload_stream_data)
            .service(get_upload)
            .route("/hey", web::get().to(manual_root))
    })
        .keep_alive(Duration::from_secs(6000))
        .bind(("127.0.0.1", 3000))?.run().await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome!")
}

#[get("/uploads/{upload_id}")]
async fn get_upload(pool: web::Data<DbPool>, path: web::Path<uuid::Uuid>) -> impl Responder {
    let id = path.into_inner();

    let upload = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");
        return get_upload_by_id(id, &mut conn);
    }).await.expect("Failed to block");

    return if upload.is_none() {
        entity_not_found(&id.to_string())
    } else {
        let body = serde_json::to_string(&upload).ok();
        if body.is_none() {
            internal_server_error("Failed to serialize upload data")
        } else {
            ok(body.unwrap())
        }
    };
}

#[post("/stream-data")]
async fn upload_stream_data(pool: web::Data<DbPool>, req_body: web::Json<StreamData>) -> impl Responder {
    let stream_data = req_body.into_inner();

    web::block(move || {
        let mut conn = pool.get().expect("failed to get connection");
        insert_upload(map_stream_data_to_upload(&stream_data, UploadStatus::Processing), &mut conn);
        insert_stream_data(&stream_data.streams, &mut conn);
        insert_upload(map_stream_data_to_upload(&stream_data, UploadStatus::Completed), &mut conn);
    }).await.expect("Failure");

    HttpResponse::Ok().body("Uploaded")
}

async fn manual_root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to MusicInsights!")
}

fn map_stream_data_to_upload(stream_data: &StreamData, status: UploadStatus) -> Upload {
    return Upload {
        upload_id: stream_data.upload_id.parse().unwrap(),
        username: Option::from(stream_data.username.clone()),
        first_stream: Option::from(stream_data.first_stream),
        last_stream: Option::from(stream_data.last_stream),
        number_of_streams: Option::from(stream_data.number_of_streams),
        status: Option::from(status.to_string()),
        message: Option::from("".to_string()),
    };
}