# Overview

Web API to save user's shopping cart data to Redis.

Built with [Actix Web](https://actix.rs/) framework.

# Features

- Redis hash data structure to store user's cart data.
- Save product ID and quantity
- Get user's cart data

# Notes

Each request to Redis creates a new connection. Keep this in mind when using.
