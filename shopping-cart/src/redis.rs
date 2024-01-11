use std::collections::BTreeMap;
use redis::{
    self,
    Commands,
    RedisResult,
};
use crate::item::CartItem;

const DEFAULT_USER: &'static str = "user1";

pub fn get_redis_connection() -> redis::Connection {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to redis: {}", e),
    };
    match client.get_connection() {
        Ok(conn) => return conn,
        Err(e) => panic!("Failed to open connection to redis: {}", e),
    };
}

pub fn set_item(conn: &mut redis::Connection, item: CartItem) -> RedisResult<()> {
    let _: () = conn.hset(DEFAULT_USER, item.product_id, item.quantity)?;
    Ok(())
}

pub fn get_items(conn: &mut redis::Connection) -> RedisResult<Vec<CartItem>> {
    let result: BTreeMap<String, u32> = conn.hgetall(DEFAULT_USER)?;
    let mut items = Vec::new();

    for (key, value) in result.into_iter() {
        items.push(CartItem {product_id: key, quantity: value});
    }

    Ok(items)
}
