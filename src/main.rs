use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;

use MusicInsightsSearchService::models::StreamData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().limit(20 * 1024 * 1024))
            .service(root)
            .service(upload_stream_data)
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

#[post("/stream-data")]
async fn upload_stream_data(req_body: web::Json<StreamData>) -> impl Responder {
    println!("{:?}", req_body);
    HttpResponse::Ok().body("Uploaded")
}

async fn manual_root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to MusicInsights!")
}