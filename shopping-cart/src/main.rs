use actix_web::{web, App, HttpServer};
use shopping_cart::item::{handle_get_items, handle_get_user_items, handle_set_user_item};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/items").route(web::get().to(handle_get_items)))
            .service(
                web::resource("/useritems/{username}")
                    .route(web::get().to(handle_get_user_items))
                    .route(web::post().to(handle_set_user_item)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
