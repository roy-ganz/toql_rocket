//! Rocket integration of Toql.
//! This contains 
//!  - A high level function to query Toql structs.
//!  - Query parameters.
//!  - Support to add counting information to HTTP response headers
//!
//! This allows to query Toql structs like this
//! 
//! ```ignore
//! #[macro_use]
//! extern crate rocket;
//! #[macro_use]
//! extern crate rocket_contrib;
//! 
//! use toql_rocket::{ToqlQuery, Counted, Result, mysql::load_many, toql::sql_mapper::SqlMapperRegistry};
//! use rocket::request::Form;
//! use rocket_contrib::json::Json;
//! use myql::Conn;
//! 
//! #[database("example_db")]
//! struct ExampleDbConnection(mysql::Conn);
//! 
//! struct User {id:u64, username: Option<String>};
//! 
//! #[get("/?<toql..>")]
//! fn query( mappers: State<SqlMapperRegistry>,
//!               conn: ExampleDbConnection, 
//!               toql: Form<ToqlQuery>)
//! -> Result<Counted<Json<Vec<User>>>> {
//!    let ExampleDbConnection(mut c) = conn;
//!
//!    let r = load_many::<User>(&toql, &mappers, &mut c)?;
//!    Ok(Counted(Json(r.0), r.1))
//! }
//! 
//! ```
//! 
//! 

pub mod counted;
pub mod query;
pub mod error;
//pub mod cache;
pub mod prelude;


pub mod template;

#[cfg(feature = "mysql")]
pub mod mysql_async;

pub use counted::Counted;
pub use query::ToqlQuery;
pub use error::Result;

pub use toql; // Reexport

