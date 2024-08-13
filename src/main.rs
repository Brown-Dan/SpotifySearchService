use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;

use MusicInsightsSearchService::{get_upload_by_id, insert_stream_data, insert_upload};
use MusicInsightsSearchService::models::{entity_not_found, internal_server_error, ok, StreamData, Upload};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().limit(20 * 1024 * 1024))
            .service(root)
            .service(upload_stream_data)
            .service(get_upload)
            .route("/hey", web::get().to(manual_root))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to MusicInsights!")
}

#[get("/uploads/{upload_id}")]
async fn get_upload(path: web::Path<uuid::Uuid>) -> impl Responder {
    let id = path.into_inner();

    return if get_upload_by_id(id).is_none() {
        entity_not_found(&id.to_string())
    } else {
        let body = serde_json::to_string(&get_upload_by_id(id)).ok();
        if body.is_none() {
            internal_server_error("Failed to serialize upload data")
        } else {
            ok(body.unwrap())
        }
    };
}

#[post("/stream-data")]
async fn upload_stream_data(req_body: web::Json<StreamData>) -> impl Responder {
    let stream_data = req_body.into_inner();
    stream_data.streams.iter().for_each(|stream| { insert_stream_data(stream) });
    insert_upload(map_stream_data_to_upload(stream_data));
    HttpResponse::Ok().body("Uploaded")
}

async fn manual_root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to MusicInsights!")
}

fn map_stream_data_to_upload(stream_data: StreamData) -> Upload {
    return Upload {
        upload_id: stream_data.upload_id.parse().unwrap(),
        username: Option::from(stream_data.username),
        first_stream: Option::from(stream_data.first_stream),
        last_stream: Option::from(stream_data.last_stream),
        number_of_streams: Option::from(stream_data.number_of_streams),
    };
}