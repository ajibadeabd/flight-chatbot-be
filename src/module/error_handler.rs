
use rocket::Request;

use crate::module::response_handler::CustomError;


#[catch(404)]
pub fn not_found(req: &Request) -> CustomError {
   let response =  format!("I couldn't find '{}'. Try something else?", req.uri());
    CustomError::NotFound(response)
}

#[catch(500)]
pub fn internal_error() -> CustomError  {
   let response =  format!("Whoops! Looks like we messed up.");
   CustomError::Internal(response)
}


