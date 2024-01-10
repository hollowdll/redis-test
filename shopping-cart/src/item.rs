use actix_web::{web, Responder, HttpResponse, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    product_id: String,
    quantity: u32,
}

impl CartItem {
    pub fn new(product_id: &str, quantity: u32) -> Self {
        Self {
            product_id: String::from(product_id),
            quantity,
        }
    }
}

pub async fn add_item(req: web::Json<CartItem>) -> Result<impl Responder> {
    println!("{:?}", req.into_inner());
    Ok(HttpResponse::NoContent())
}

pub async fn get_items() -> Result<impl Responder> {
    let mut items = Vec::new();
    items.push(CartItem::new("plate", 3));
    items.push(CartItem::new("bike", 1));
    items.push(CartItem::new("banana", 5));

    Ok(web::Json(items))
}
