use toql::query::Query;
use toql::page::Page;
use toql::result::Result;
use toql::query_parser::QueryParser;
use rocket::FromForm;
use std::fmt;




/// Struct to hold URL query parameters
/// 
/// This allows to query entities with optional query parameters.
/// Ensure that you URL encode your query! 
/// Instead of `query=*` you must write `query=%2A`.
/// 
/// ### Examples of URL queries for Toql
/// ```url
///  ..?query=id%2C%20%2Busername&first=5&max=20&count=false
///  ..?max=5
///  ..?query=%2A,phone_%2A,count=false
/// ```
#[derive(FromForm, Debug)]
pub struct ToqlQuery {
    /// The actual Toql query, like `id, +username, phone_*`
    /// 
    /// Default `Some("*")`
    pub query: Option<String>,
  
    /// The offset to the first record. For example 10 will skip the first 10 records.
    /// 
    /// Default `Some(0)`
    pub first: Option<u64>,
    /// The maximum number of records to return.
    /// 
    /// Default `Some(10)`
    pub max: Option<u16>,
    /// Get filtered count and total count.
    /// 
    /// Default `Some(true)`
    pub count: Option<bool>,

}

type ParsedQuery<M> = (Query<M>,Page);


impl ToqlQuery {
    pub fn parse_query<M>(&self) -> Result<Query<M>> {

        let query = QueryParser::parse::<M>( self.query.as_ref().map_or("*", |q|q.as_str()))?;
        Ok(query)
    }

   pub fn parse<M>(&self) -> Result<ParsedQuery<M>> {

        let query = self.parse_query()?;
        let first = self.first.unwrap_or(0);
        let max = self.max.unwrap_or(10);
        let page = if self.count.unwrap_or(true) {
            Page::Counted(first,max )
        } else {
            Page::Uncounted(first, max)
        };

        Ok((query, page))
    }
}


impl fmt::Display for ToqlQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.query {
            Some(q) => q.fmt(f),
            None => write!(f,"*")
        }
    }
}
 