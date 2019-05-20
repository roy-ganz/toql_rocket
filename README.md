# Toql Rocket

### Description
Use [Toql](https://crates.io/crates/toql) with [Rocket](https://crates.io/crates/rocket). This crate adds:

- Toql query parameters
- HTTP response for Toql errors
- `load_many` function that works with query parameters
- Response headers with count information

``` ignore
--snip--
use toql_rocket::{Result, ToqlQuery, load_many, Counted, toql::sql_mapper::SqlMapperCache};

#[get("/?<toql..>")]
pub fn query( mappers: State<SqlMapperCache>, conn: ExampleDbConnection, toql: Form<ToqlQuery>) 
    -> Result<Counted<Json<Vec<User>>>> 
{
    let ExampleDbConnection(mut c) = conn;

    let r = load_many::<User>(&toql, &mappers, &mut c)?;
    Ok(Counted(Json(r.0), r.1))
}
```

### Resources
Check out the [CRUD example](https://github.com/roy-ganz/toql_rocket/blob/master/examples/crud_mysql/main.rs). 


### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toql_rocket = { version = "0.1", features= ["mysql"] }

```





## License

Toql Rocket is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

