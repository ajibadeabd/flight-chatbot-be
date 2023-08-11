use rocket::{local::blocking::Client, http::{Status, ContentType}};
use serde_json;
use lazy_static::lazy_static;
use std::sync::Mutex;

// Import your Rocket application and AppState type here
use crate::{rocket, module::{response_handler::GenericResponse, route_structure::{FlightOption, BookingResponse}}};



lazy_static! {
    static ref FLIGHT_ID: Mutex<String> = Mutex::new(String::new());
    static ref BOOKING_ID: Mutex<String> = Mutex::new(String::new());
}

#[test]
fn test_get_flight_schedule() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/flights?limit=1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let response_body = response.into_string().expect("Response Body");
    let flight_response: GenericResponse<Vec<FlightOption>>  = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(flight_response.data.len(),1);
    assert_eq!(flight_response.data.len(),1);
    assert_eq!(flight_response.message,"Data retrieved successfully.");

    
     
}

#[test]
fn test_booking_with_wrong_id() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    // Create a JSON payload for booking
    let payload = serde_json::json!({
        "flight_id": "111",
        "passenger_name":"kord",
        "email":"kord@ss.com"
    });

    let response = client.post("/api/booking").json(&payload).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    
}

#[test]
fn test_booking_with_right_id() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    // Create a JSON payload for booking
    let payload = serde_json::json!({
        "flight_id": "504",
        "passenger_name":"kord",
        "email":"kord@ss.com"
    });

    let response = client.post("/api/booking").json(&payload).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.into_string().expect("Response Body");
    
    let flight_response: GenericResponse<BookingResponse>  =
     serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let mut shared_book_id = BOOKING_ID.lock().expect("Mutex lock failed");
    *shared_book_id = flight_response.data.booking_id.clone();
 
 
 }
 

#[test]
fn test_get_payment_page() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let payment_id = "some_payment_id";

    let response = client.get(format!("/payment/{}", payment_id)).dispatch();
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(response.status(), Status::Ok);
    let  expected_response_body = "<html><body><h1>Invalid payment link.</h1></body></html>".to_owned();

    assert_eq!(response.into_string().unwrap(), expected_response_body);
}

#[test]
fn test_post_payment_page() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    
    // Create a JSON payload for payment callback
    let payload = serde_json::json!({
        "payment_id": "wrong payment_id"
    });

    let response = client.post("/api/payment").json(&payload).dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    // Add more assertions for response body or other details if needed
}
