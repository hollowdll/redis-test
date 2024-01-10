use actix_web::{get, web, App, HttpServer, Responder};
use shopping_cart::item::{
    get_items,
    add_item,
};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/items")
                .route(web::get().to(get_items))
                .route(web::post().to(add_item))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
