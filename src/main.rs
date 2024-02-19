use sabinnylol::{index, search};
use actix_web::{App, HttpServer, Responder, get, post, web, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

 
