use actix_web::{web, Responder, HttpResponse, Result, error};
use crate::redis::*;
use serde::{Serialize, Deserialize};

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

pub async fn handle_set_user_item(req: web::Json<CartItem>) -> Result<impl Responder> {
    let mut conn = get_redis_connection();
    let _ = set_item(&mut conn, req.into_inner()).map_err(error::ErrorInternalServerError)?;
    println!("User item set successfully");

    Ok(HttpResponse::NoContent())
}

pub async fn handle_get_user_items() -> Result<impl Responder> {
    let mut conn = get_redis_connection();
    let items = get_items(&mut conn).map_err(error::ErrorInternalServerError)?;
    println!("User items got successfully");

    Ok(web::Json(items))
}
