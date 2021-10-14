use rocket::http::Status;
use rocket::Response;
use rocket::Request;
use rocket::response::Responder;
use toql::prelude::PageCounts;

/// Struct that keeps a result with counting information.
#[derive( Debug)]
pub struct Counted<R>(pub R, pub Option<PageCounts>);

/// Creates a response and puts the counting information
/// into the HTTP response headers, if there is any.
/// 
/// The header `X-Total-Count` contains the title value of the page counts in [Counted].
/// The header `X-Filtered-Count` contains the filtered value of the page counts in [Counted].
impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for Counted<R> 
{
    fn respond_to(self, req: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let mut build = Response::build();
        let responder = self.0;
        build.merge(responder.respond_to(req)?);

        if let Some(PageCounts{filtered, total}) = self.1 {
            build.raw_header("X-Total-Count", total.to_string());
            build.raw_header("X-Filtered-Count", filtered.to_string());
        }
           
         build.ok()
    }
}