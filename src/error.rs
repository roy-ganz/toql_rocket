
use toql::error::ToqlError;
use std::ops::Deref;
use rocket::http::Status;
use rocket::Response;
use rocket::Request;
use std::io::Cursor;

 macro_rules! bad_request_template {
    ($description:expr) => (
        format!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>400 Bad Request</title>
            </head>
            <body align="center">
                <div align="center">
                    <h1>400: Bad Request</h1>
                    <p>Request failed, because {}.</p>
                    <hr />
                    <small>Rocket</small>
                </div>
            </body>
            </html>
        "#, $description
        )
    )
}


/// A result with a [`ToqlError`](enum.ToqlError.html)
pub type Result<T> = std::result::Result<T, ToqlErrorWrapper>;



/// Wrapper for [ToqlError]
/// Needed for trait implementation.
#[derive(Debug)]
pub struct ToqlErrorWrapper (ToqlError);

impl From<ToqlError> for ToqlErrorWrapper{
        fn from(err: ToqlError) -> ToqlErrorWrapper {
            ToqlErrorWrapper(err)
    }
}


impl rocket::response::Responder<'static> for ToqlErrorWrapper {

    fn respond_to(self, _: &Request) -> std::result::Result<Response<'static>, Status> {
        let mut response = Response::new();
      

        match self.0 {
            ToqlError::NotFound => {
                log::info!("No result found");
                Err(Status::NotFound)
            }
            ToqlError::SqlBuilderError(err) => {
                log::info!("{}", err);
                response.set_status(Status::BadRequest);
                response.set_sized_body(Cursor::new(bad_request_template!(err)));
                Ok(response)
            }
              ToqlError::EncodingError(err) => {
                log::info!("{}", err);
               response.set_status(Status::BadRequest);
                response.set_sized_body(Cursor::new(bad_request_template!(err)));
                Ok(response)
             }
             ToqlError::QueryParserError(err) => {
                log::info!("{}", err);
                response.set_status(Status::BadRequest);
                response.set_sized_body(Cursor::new(bad_request_template!(err)));
                Ok(response)
             }
            ToqlError::NotUnique => {
                log::info!("No unique result found");
                response.set_status(Status::BadRequest);
                response.set_sized_body(Cursor::new(bad_request_template!("no unique record found")));
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
