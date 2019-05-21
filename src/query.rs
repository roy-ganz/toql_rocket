use toql::query::Query;
use toql::error::Result;
use toql::query_parser::QueryParser;
use rocket::FromForm;
use rocket::http::RawStr;
use toql::error::ToqlError;
use rocket::request::FromFormValue;
use std::ops::Deref;



/// Wrapper for [Query]
/// Needed for trait implementation.
#[derive(Debug)]
pub struct QueryWrapper (pub toql::query::Query);

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
    pub query: Option<QueryWrapper>,
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


impl ToqlQuery {

    pub  fn and(mut self, query:Query) -> Self {
        let x = self.query.unwrap_or(QueryWrapper(Query::wildcard())).0.parenthesize().and(query);
        self.query = Some(QueryWrapper(x));
        self
    }

}



impl<'v> FromFormValue<'v> for QueryWrapper {
    type Error = ToqlError;

    fn from_form_value(form_value: &'v RawStr) -> Result<QueryWrapper> {
       
     
       if form_value.len() == 0 {
            return Ok(QueryWrapper(Query::wildcard()));  
       }
       let url = form_value.url_decode();
       match  url {
           Err(err) => Err(ToqlError::EncodingError(err)),
           Ok(u) =>  {
                let q = QueryParser::parse(&u)?;
                Ok(QueryWrapper(q))    
           }
       }
    }
} 

/// Unwrap on deref
impl Deref for QueryWrapper {
    type Target = Query;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}