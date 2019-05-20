# Toql Rocket

### Description
Use [Toql](https://crates.io/crates/toql) with [Rocket](https://crates.io/crates/rocket). This crate adds:

- Toql URL query parameters
- HTTP response for Toql errors
- `load_many()` function that works with URL query parameters
- Response headers with count information


### Resources
There is a  [CRUD example](https://github.com/roy-ganz/toql_rocket/blob/master/examples/crud_mysql/main.rs) and the [API documentation](https://docs.rs/toql_rocket/0.1/toql/)


### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toql_rocket = { version = "0.1", features= ["mysql"] }

```





## License

Toql Rocket is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

