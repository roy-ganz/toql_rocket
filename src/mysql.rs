
use crate::query::ToqlQuery;
use crate::query::QueryWrapper;

use toql::query::Query;
use toql::error::Result;
use toql::sql_mapper::SqlMapperCache;
use toql::mysql::load::Load;
use toql::mysql::mysql::Conn;


/// Facade function to query structs with URL query parameters from a MySQL database.
/// 
/// This needs the MySQL feature enabled.
pub fn load_many<'a, T: Load<T>>(
    toql_query: &ToqlQuery,
    mappers: &SqlMapperCache,
    mut conn: &mut Conn
) 
-> Result<(Vec<T>, Option<(u32, u32)>)>
{
    // Returns sql errors
    T::load_many(
        &toql_query.query.as_ref().unwrap_or(&QueryWrapper(Query::wildcard())).0,
        &mappers,
        &mut conn,
        toql_query.count.unwrap_or(true),
        toql_query.first.unwrap_or(0),
        toql_query.max.unwrap_or(10),
    )
}

