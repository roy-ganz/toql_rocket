








use toql::error::ToqlError;
use toql_mysql_async::error::ToqlMySqlAsyncError;
use rocket::{Request, Response, http::Status};
use std::{ops::Deref, io::Cursor};

use crate::bad_request_template;

/// A result with a [`ToqlError`](enum.ToqlError.html)
pub type Result<T> = std::result::Result<T, ToqlMySqlAsyncErrorWrapper>;

/// Wrapper for [ToqlError]
/// Needed for trait implementation.
#[derive(Debug)]
pub struct ToqlMySqlAsyncErrorWrapper (ToqlMySqlAsyncError);

impl From<ToqlMySqlAsyncError> for ToqlMySqlAsyncErrorWrapper{
        fn from(err: ToqlMySqlAsyncError) -> ToqlMySqlAsyncErrorWrapper {
            ToqlMySqlAsyncErrorWrapper(err)
    }
}

impl From<ToqlError> for ToqlMySqlAsyncErrorWrapper{
        fn from(err: ToqlError) -> ToqlMySqlAsyncErrorWrapper {
            ToqlMySqlAsyncErrorWrapper(ToqlMySqlAsyncError::ToqlError(err))
    }
}



impl<'r> rocket::response::Responder<'r, 'static> for ToqlMySqlAsyncErrorWrapper {

    fn respond_to(self, request: &'r Request<'_>) -> std::result::Result<Response<'static>, Status> {
        let mut response = Response::new();
      

        match self.0 {
            ToqlMySqlAsyncError::ToqlError(err) => {
                 crate::error::ToqlErrorWrapper(err).respond_to(request)
            }
            ToqlMySqlAsyncError::MySqlError(err) => {
               log::info!("{}", err);
               response.set_status(Status::BadRequest);
               let msg = bad_request_template!(err);
               response.set_sized_body(msg.len(), Cursor::new(msg));
               Ok(response)
            }
            ToqlMySqlAsyncError::FromValueError(err) => {
               log::info!("{}", err);
               response.set_status(Status::BadRequest);
               let msg = bad_request_template!(err);
               response.set_sized_body(msg.len(), Cursor::new(msg));
               Ok(response)
            }
            
        }

    }
}


/// Unwrap on deref
impl Deref for ToqlMySqlAsyncErrorWrapper {
    type Target = ToqlMySqlAsyncError;

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
 toql::mysql::MySql<'a,C>: Load<T, Error=ToqlMySqlAsyncError>,  T: toql::key::Keyed,
{
    // parse query



    
   

    // Returns sql errors
    mysql_toql.load_page::<T>(query.query, query.page)
}

 */