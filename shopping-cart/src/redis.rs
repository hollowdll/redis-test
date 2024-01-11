use crate::item::CartItem;
use redis::{self, Commands, RedisResult};
use std::collections::BTreeMap;

pub fn get_redis_connection() -> redis::Connection {
    let client = match redis::Client::open("redis://127.0.0.1:6379/") {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to redis: {}", e),
    };
    match client.get_connection() {
        Ok(conn) => return conn,
        Err(e) => panic!("Failed to open connection to redis: {}", e),
    };
}

pub fn set_item(conn: &mut redis::Connection, username: String, item: CartItem) -> RedisResult<()> {
    let _: () = conn.hset(
        format!("user:{}:cart", username),
        item.product_id,
        item.quantity,
    )?;
    Ok(())
}

pub fn get_items(conn: &mut redis::Connection, username: String) -> RedisResult<Vec<CartItem>> {
    let result: BTreeMap<String, u32> = conn.hgetall(format!("user:{}:cart", username))?;
    let mut items = Vec::new();

    for (key, value) in result.into_iter() {
        items.push(CartItem {
            product_id: key,
            quantity: value,
        });
    }

    Ok(items)
}
