
//! Use [Toql](https://crates.io/crates/toql) with [Rocket](https://crates.io/crates/rocket). 
//! 
//! This crate adds:
//! 
//! - Toql URL query parameters
//! - Response headers with page count information
//! - Responder for [ToqlError](toql::prelude::ToqlError)
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! toql_rocket = "0.3"
//! ```
//! 
//! Then enjoy some simplifications in  your rocket handlers:
//! ```rust
//! use toql_rocket::prelude::{ToqlQUery, Counted};
//! #[get("/?<toql_query..>")]
//! pub async fn query(
//!     cache: &State<Cache>,
//!     mut conn: Connection<TodoDb>,
//!     toql_query: ToqlQuery,  //<!-- Get URL parameters with default values
//! ) -> Result<Counted<Json<Vec<Todo>, ServerError>>> 
//! {
//!     let mut toql = MySqlAsync::from(&mut *conn, &*cache);
//!     let (query, page) = toql_query.parse::<Todo>()?; //<!-- Parse into typesafe query
//!     let r = toql.load_page(query, page).await?;
//! 
//!     Ok(Counted(Json(r.0), r.1)) //<!-- Put page count information into headers
//! }
//! ```
//! 
//! Check out the full featured [REST server](https://github.com/roy-ganz/todo_rotomy) based on Rocket, Toql and MySQL.
//! 
//! 
//! 
//! ## License
//! Toql Rocket is distributed under the terms of both the MIT license and the
//! Apache License (Version 2.0).

pub mod counted;
pub mod query;
pub mod error;
pub mod prelude;
pub mod template;

#[cfg(feature = "mysql")]
pub mod mysql_async;

pub use toql; // Reexport

