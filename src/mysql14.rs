








use toql::mysql::error::ToqlMySqlError;
use toql::error::ToqlError;


use std::ops::Deref;
use rocket::http::Status;
use rocket::Response;
use rocket::Request;
use std::io::Cursor;

use crate::bad_request_template;

/// A result with a [`ToqlError`](enum.ToqlError.html)
pub type Result<T> = std::result::Result<T, ToqlMySqlErrorWrapper>;

/// Wrapper for [ToqlError]
/// Needed for trait implementation.
#[derive(Debug)]
pub struct ToqlMySqlErrorWrapper (ToqlMySqlError);

impl From<ToqlMySqlError> for ToqlMySqlErrorWrapper{
        fn from(err: ToqlMySqlError) -> ToqlMySqlErrorWrapper {
            ToqlMySqlErrorWrapper(err)
    }
}

impl From<ToqlError> for ToqlMySqlErrorWrapper{
        fn from(err: ToqlError) -> ToqlMySqlErrorWrapper {
            ToqlMySqlErrorWrapper(ToqlMySqlError::ToqlError(err))
    }
}



impl rocket::response::Responder<'static> for ToqlMySqlErrorWrapper {

    fn respond_to(self, request: &Request) -> std::result::Result<Response<'static>, Status> {
        let mut response = Response::new();
      

        match self.0 {
            ToqlMySqlError::ToqlError(err) => {
                 crate::error::ToqlErrorWrapper(err).respond_to(request)
            }
            ToqlMySqlError::MySqlError(err) => {
               log::info!("{}", err);
               response.set_status(Status::BadRequest);
               response.set_sized_body(Cursor::new(bad_request_template!(err)));
               Ok(response)
            }
            
        }

    }
}


/// Unwrap on deref
impl Deref for ToqlMySqlErrorWrapper {
    type Target = ToqlMySqlError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/* 
/// Facade function to query structs with URL query parameters from a MySQL database.
/// 
/// This needs the MySQL feature enabled.
pub fn load_page<'a, T,C: 'a >(
    query: &ParsedQuery,
    mut mysql_toql: &mut toql::mysql::MySql<'a, C>
) 
-> Result<(Vec<T>, Option<(u32, u32)>)>
where C :GenericConnection,
 toql::mysql::MySql<'a,C>: Load<T, Error=ToqlMySqlError>,  T: toql::key::Keyed,
{
    // parse query



    
   

    // Returns sql errors
    mysql_toql.load_page::<T>(query.query, query.page)
}

 */