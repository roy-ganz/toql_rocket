//! Wrapper for [ToqlError](toql::error::ToqlError) that implements [Responder](rocket::response::Responder)

use toql::error::ToqlError;
use std::ops::Deref;
use rocket::http::Status;
use rocket::Response;
use rocket::Request;
use std::io::Cursor;

use crate::bad_request_template;


/// Wrapper for [ToqlError] to satisfy orphan rule.
/// This implements a responder for ToqlError. 
/// It will return 
/// - 404 for not found
/// - 400 for parser mistakes
/// - 500 for internal errors
/// Newtype needed for orphan rule.
#[derive(Debug)]
pub struct ToqlErrorWrapper ( pub ToqlError);

impl From<ToqlError> for ToqlErrorWrapper{
        fn from(err: ToqlError) -> ToqlErrorWrapper {
            ToqlErrorWrapper(err)
    }
}


impl<'r> rocket::response::Responder<'r, 'static> for ToqlErrorWrapper {

    fn respond_to(self, _: &'r Request<'_>) -> std::result::Result<Response<'static>, Status> {
        let mut response = Response::new();
      
        match self.0 {
            ToqlError::NotFound => {
                log::info!("No result found");
                Err(Status::NotFound)
            }
            ToqlError::SqlBuilderError(err) => {
                log::info!("{}", err);
                response.set_status(Status::BadRequest);
                let msg = bad_request_template!(err);
                response.set_sized_body(msg.len(), Cursor::new(msg));
                Ok(response)
            }
              ToqlError::EncodingError(err) => {
                log::info!("{}", err);
               response.set_status(Status::BadRequest);
               let msg = bad_request_template!(err);
                response.set_sized_body(msg.len(), Cursor::new(msg));
                Ok(response)
             }
             ToqlError::QueryParserError(err) => {
                log::info!("{}", err);
                response.set_status(Status::BadRequest);
                let msg = bad_request_template!(err);
                response.set_sized_body(msg.len(), Cursor::new(msg));
                Ok(response)
             }
            ToqlError::NotUnique => {
                log::info!("No unique result found");
                response.set_status(Status::BadRequest);
                let msg= bad_request_template!("no unique record found");
                response.set_sized_body(msg.len(),Cursor::new(msg));
                Ok(response)
            },
            err => {
                log::error!("Toql failed with `{}`",err);
                Err(rocket::http::Status::InternalServerError)
            }
        }

    }
}


/// Unwrap on deref
impl Deref for ToqlErrorWrapper {
    type Target = ToqlError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
