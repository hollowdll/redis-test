# Overview

Web API to save user's shopping cart data to Redis.

Built with [Actix Web](https://actix.rs/) framework.

# Features

- Redis hash data structure to store user's cart data.
- Save product ID and quantity
- Get user's cart data

# Notes

Each request to Redis creates a new connection. Keep this in mind when using.

# Build

You need the Rust compiler to build the application.

Install here https://www.rust-lang.org/tools/install

# Usage

Easy way to spin up a local Redis instance is to use Docker.

https://hub.docker.com/_/redis

```bash
docker pull redis
```
Start a new container and run it in the background
```bash
docker run -p 6379:6379 --name redis-test -d redis
```
This maps the container's port `6379` to your host's port `6379`. Port `6379` is the default Redis port that the application uses.

Start the application. It runs in port `8080`.
```bash
cargo run
```

Save item for user `user1`
```bash
curl -X POST -H "Content-Type: application/json" -d '{"product_id": "product1", "quantity": 3}' http://127.0.0.1:8080/useritems/user1
```

Get user's `user1` cart data
```bash
curl http://127.0.0.1:8080/useritems/user1
```
