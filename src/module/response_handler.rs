use serde::{Serialize, Deserialize};
use std::io::Cursor;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;


#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}

 
#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}



//#[derive(Error)]
#[derive( Debug, Clone)]
pub enum CustomError {
    //#[resp("{0}")]
    Internal(String),

    //#[resp("{0}")]
    NotFound(String),

    //#[resp("{0}")]
    BadRequest(String),
}

impl CustomError {
    fn get_http_status(&self) -> Status {
        match self {
            CustomError::Internal(_) => Status::InternalServerError,
            CustomError::NotFound(_) => Status::NotFound,
            _ => Status::BadRequest,
        }
    }
    fn get_http_error(&self) -> String {
        match self {
            CustomError::Internal(err) =>err.to_owned(),
            CustomError::NotFound(err) => err.to_owned(),
            CustomError::BadRequest(err) => err.to_owned(),
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_error())
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // serialize struct into json string
        let err_response = serde_json::to_string(&ErrorResponse{
            message: self.to_string(),
            status: "failed".to_owned(),
        }).unwrap();
        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
 
#[derive(Serialize)]
pub struct CustomResult {
    pub result: String,
    pub status_code:Option<u16>
}

 
 
 

impl std::fmt::Display for CustomResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.result)
    }
}
 

impl<'r> Responder<'r, 'static> for CustomResult {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status { code: self.status_code.unwrap()})
            .header(ContentType::JSON)
            .sized_body(self.result.len(), Cursor::new(self.result))
            .ok()
    }
}
 
pub fn generic_response<T>(message: &str, data: Option<T>,code:Option<u16>) -> CustomResult 
where
    T: serde::Serialize,
{
    let response_json = GenericResponse {
        message: message.to_string(),
        status: "success".to_string(),
        data,
    };

    let json_response = serde_json::to_string(&response_json).unwrap();
    CustomResult { result: json_response ,status_code:Some(code.unwrap_or(200))
    }
}

 