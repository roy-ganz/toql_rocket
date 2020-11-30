#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use rocket::{State,request::Form,http::Status};
use rocket_contrib::{databases::mysql, json::Json};
use serde::{Serialize, Deserialize};

//use toql::derive::Toql;
// use toql_rocket::{ToqlQuery, Counted, Result,};

use toql_rocket::prelude::{ToqlQuery, Counted, Result, Cache, query, fields, paths, MySql, Toql};  // or simple toql_rocket::prelude::*

// Here is our struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Toql)]
pub struct Todo {
    #[serde(skip_deserializing, default)]
    #[toql(key)] 
    pub id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub completed: Option<bool>,
}

// Here come the crud functions
#[delete("/<id>")]
pub fn delete<'a>(id: u64, conn: ExampleDbConnection, cache: State<Cache>) -> Result<Status> {
    
    let ExampleDbConnection(mut c ) = conn;
    let toql = MySql::from(&mut *c, &*cache);
   
    let affected_rows= toql.delete_one(TodoKey::from(id))?;  // TodoKey is generated from Toql derive
        
    if affected_rows == 0 {
        return Ok(Status::NotFound);
    }
    Ok(Status::NoContent)
}

#[put("/<id>", data = "<todo>")]
pub fn update(
    id: u64,
    mut todo: Json<Todo>,
    cache: State<Cache>,
    conn: ExampleDbConnection,
) -> Result<Json<Todo>> {
    
    
    let toql = MySql::from(&mut &*conn, &*cache);
  
    let affected_rows = toql.update_one(fields!(Todo, "*"), &todo.into_inner())?; // Consider all possible fields for update, no nested update
    let u = toql.load_one(query!(Todo, "*, id eq ?", id))?;
    Ok(Json(u))
}

#[post("/", data = "<todo>")] // format = "application/json",
pub fn create<'a>(
    todo: Json<Todo>,
    cache: State<Cache>,
    conn: ExampleDbConnection,
) -> Result<Json<Todo>> {
    
    let toql = MySql::from(&mut *conn, &*cache);

    toql.insert_one(paths!(Todo,""), &todo.into_inner())?; // `paths` specify which nested structs we also wand to insert - `Todo` has none

    let u = toql.load_one(query!(Todo, "*, id eq ?", &todo.id))?; 
    Ok(Json(u))
}

#[get("/<id>")]
pub fn get(
    id: u64,
    cache: State<Cache>,
    conn: ExampleDbConnection,
) -> Result<Json<Todo>> {
    let ExampleDbConnection(mut c) = conn;
    let toql =  MySql::from(&mut *c, &*cache);
   
    let u = toql.load_one(query!(Todo, "*, id eq ?", id))?;
    Ok(Json(u))
}

#[get("/?<toql_query..>")]
pub fn query(
    cache: State<Cache>,
    conn: ExampleDbConnection,
    toql_query: Form<ToqlQuery>,
) -> Result<Counted<Json<Vec<Todo>>>> {
    let ExampleDbConnection(mut c) = conn;
    let (query, page) = toql_query.parse::<Todo>()?; 
    let toql =  MySql::from(&mut *c, &*cache);

    let r = toql.load_page(query, page)?;
    Ok(Counted(Json(r.0), r.1))
}

// The database connection
#[database("example_db")]
pub struct ExampleDbConnection(mysql::Conn);



// Main to startup the server
fn main() {
    println!("------------------------------------------");
    println!("Full Toql CRUD example with Rocket / MySql");
    println!("------------------------------------------");
    println!("This example assumes that you have a MySql Server");
    println!("running with a database `example_db`");
    println!("Run the following SQL to create the table `Todo`");
    println!("CREATE TABLE `Todo` (`id` int(11) NOT NULL AUTO_INCREMENT,`title` varchar(200) NOT NULL, `completed` tinyint(1) PRIMARY KEY (id))");
    println!("------------------------------------------------------------------------------------------------------------");
    println!("Start the server with ");
    println!("ROCKET_DATABASES={{example_db={{url=mysql://USER:PASS@localhost:3306/example_db}}}} cargo +nightly run --example=\"crud_mysql\" --features=\"mysql\"");
    println!("----------------------------------------------------------------------------------------------------------------------");
    println!(
        "Create a todo with `curl localhost:8000/todo -X POST -d '{{\"title\":\"Water plants\"}}'`"
    );
    println!(
        "Update a todo with `curl localhost:8000/todo/ID -X PUT -d '{{\"completed\":\"true\"}}'`"
    );
    println!("Get a single todo with `curl localhost:8000/todo/ID`");
    println!("Get all todos with `curl localhost:8000/todo`");
    println!("Get only completed todos in descending order `curl localhost:8000/todo?query=-id,completed+eq+1`");
    println!("Delete a todo with `curl -X DELETE localhost:8000/todo/ID`");
    println!("--------------------------");



    
    let mut cache = Cache::new(); // Toql cache for mapping information

    rocket::ignite()
        .manage(cache)
        .attach(ExampleDbConnection::fairing())
        .mount("/todo", routes![get, query, create, update, delete])
        .launch();
}


