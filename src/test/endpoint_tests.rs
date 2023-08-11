use rocket::{local::blocking::Client, http::{Status, ContentType}};
use serde_json;

// Import your Rocket application and AppState type here
use crate::{rocket};

#[test]
fn test_get_flight_schedule() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/flights").dispatch();
    assert_eq!(response.status(), Status::Ok);
     
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
 }

#[test]
fn test_payment_initiate() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let booking_id = "some_booking_id";

    let response = client.get(format!("/api/initialize_payment/{}", booking_id)).dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

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
    // Add more assertions for response body or other details if needed
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
