use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn entrypoint(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("entrypoint")
}

#[post("/post1")]
async fn post1(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey")
}

async fn yeah() -> impl Responder {
    HttpResponse::Ok().body("yeah")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(entrypoint)
            .route("/hey", web::get().to(hey)) 
            .route("/yeah", web::get().to(yeah)) 
    })
    .bind(("127.0.0.1", 8888))?.run().await
}
