use crate::redis::*;
use actix_web::{error, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: u32,
}

impl CartItem {
    pub fn new(product_id: &str, quantity: u32) -> Self {
        Self {
            product_id: String::from(product_id),
            quantity,
        }
    }
}

pub async fn handle_get_items() -> Result<impl Responder> {
    let mut items = Vec::new();
    items.push(CartItem::new("plate", 3));
    items.push(CartItem::new("bike", 1));
    items.push(CartItem::new("banana", 5));
    println!("Items got successfully");

    Ok(web::Json(items))
}

pub async fn handle_set_user_item(
    username: web::Path<String>,
    req: web::Json<CartItem>,
) -> Result<impl Responder> {
    let mut conn = get_redis_connection();
    let _ = set_item(&mut conn, username.into_inner(), req.into_inner())
        .map_err(error::ErrorInternalServerError)?;
    println!("User item set successfully");

    Ok(HttpResponse::NoContent())
}

pub async fn handle_get_user_items(username: web::Path<String>) -> Result<impl Responder> {
    let mut conn = get_redis_connection();
    let items =
        get_items(&mut conn, username.into_inner()).map_err(error::ErrorInternalServerError)?;
    println!("User items got successfully");

    Ok(web::Json(items))
}
