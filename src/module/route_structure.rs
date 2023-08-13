// crate::module;


///  Will be important for validation....
use rocket_validation::{Validate};

use std::{fmt};
use serde::{Serialize, Deserialize};
use rocket::{http::{uri::fmt::{UriDisplay, Query, Formatter,FromUriParam,}},};
use rocket::request::{Outcome, FromRequest};
use rocket::{Request};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
    pub count: u32,
    pub total: u32,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct Departure {
    pub airport:  Option<String>,
    pub timezone: Option<String>,
    pub iata: Option<String>,
    pub icao:  Option<String>,
    pub terminal: Option<String>,
    pub gate: Option<String>,
    pub delay: Option<i32>,
    pub scheduled:  Option<String>,
    pub estimated: Option<String>,
    pub actual: Option<String>,
    pub estimated_runway: Option<String>,
    pub actual_runway: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct Arrival {
    pub airport:  Option<String>,
    pub timezone:  Option<String>,
    pub iata:  Option<String>,
    pub icao:  Option<String>,
    pub terminal: Option<String>,
    pub gate: Option<String>,
    pub baggage: Option<String>,
    pub delay: Option<i32>,
    pub scheduled:  Option<String>,
    pub estimated:  Option<String>,
    pub actual: Option<String>,
    pub estimated_runway: Option<String>,
    pub actual_runway: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct Airline {
    pub name: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct Flight {
    pub number: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    pub codeshared: Option<CodeShared>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct CodeShared {
    pub airline_name: Option<String>,
    pub airline_iata: Option<String>,
    pub airline_icao: Option<String>,
    pub flight_number: Option<String>,
    pub flight_iata: Option<String>,
    pub flight_icao: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct FlightData {
    pub flight_date: Option<String>,
    pub flight_status:  Option<String>,
    pub departure: Departure,
    pub arrival: Arrival,
    pub airline: Airline,
    pub flight: Flight,
    pub aircraft: Option<String>,
    pub live: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct ApiResponse {
    // pub pagination: Pagination,
    pub data: Vec<FlightData>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Booking {
    pub id:  String,
    pub flight_number: String,
    pub passenger_name: String,
    pub email: String,
    pub amount: f64,
    pub payment_details:Option<Payment>
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct Payment {
    pub id: String,
    pub transaction_reference:  String,
    pub amount: f64,
    pub currency: String,
    pub email: String,
    pub status: TransactionType,
    pub timestamp: String,
}

#[derive(Debug, Clone,  Serialize,  Deserialize,PartialEq)]
pub enum TransactionType {
    // #[serde(rename = "deposit")]
     SUCCESS,
    // #[serde(rename = "withdrawal")]
     FAILED,
    // #[serde(rename = "transfer")]
    PENDING,
}

// #[derive(FromForm,Seri)]
#[derive(Debug,FromForm, Deserialize, Serialize,Clone,Validate)]
///  Implements `Validate`
#[serde(crate = "rocket::serde")]
pub struct FlightQueryParams {
    pub departure_city: Option<String>,
    pub destination_city: Option<String>,

    //#[validate(length(min = 1))] 
    pub date: Option<String>,
   // #[validate(range(min = 0, max = 100))]
    pub page: Option<i32>,
   // #[validate(range(min = 0, max = 100))]
    pub limit:Option<i32>,
}




#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct FlightIdData {
    pub flight_id:String,
    pub passenger_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct PaymentCallbackUrl {
    pub payment_id: String
}

#[derive( Deserialize, Serialize ,Debug)]
pub struct BookingResponse {
    pub booking_id: String
}




impl FromUriParam<Query, (&str, &str, &str,  &i32,  &i32)> for FlightQueryParams {
    type Target = FlightQueryParams;

    fn from_uri_param(
        (departure_city, destination_city, date, page, limit): (&str, &str, &str,   &i32, &i32),
    ) -> FlightQueryParams {
        FlightQueryParams {
            departure_city: Some(departure_city.to_owned()),
            destination_city: Some(destination_city.to_owned()),
            date: Some(date.to_owned()),
            page: Some(page.to_be()),
            limit: Some(limit.to_be()),
        }
    }
}



impl UriDisplay<Query> for FlightQueryParams {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("departure_city", &self.departure_city)?;
        f.write_named_value("destination_city", &self.destination_city)?;
        f.write_named_value("date", &self.date)?;
        f.write_named_value("page", &self.page)?;
        f.write_named_value("limit", &self.limit)
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for FlightQueryParams {
    // type Error = Infallible;
    type Error = ();

    // async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
    async fn from_request(req: &'r Request<'_>) ->  Outcome<Self, ()> {
         
        // For example:
        let departure_city = req.query_value("departure_city").and_then(|v| v.ok());
        let destination_city = req.query_value("destination_city").and_then(|v| v.ok());
        let date = req.query_value("date").and_then(|v| v.ok());
        let limit = req.query_value("limit").and_then(|v| v.ok()); // Use "limit" here
        let page = req.query_value("page").and_then(|v| v.ok()); // Use "page" here
        
        let flight_query_params = FlightQueryParams {
            departure_city: departure_city.unwrap_or_default(),
            destination_city: destination_city.unwrap_or_default(),
            limit: limit.unwrap_or_default(),
            page: page.unwrap_or_default(),
            date: date.unwrap_or_default(),
        };

        // Return Outcome::Success with the created instance
        Outcome::Success(flight_query_params)

    }
}



 

#[derive(Debug,Serialize,Deserialize)]
pub struct FlightOption {
    // pub option_number: usize,
    pub flight_date: Option<String>,
    pub scheduled_departure: Option<String>,
    pub scheduled_arrival: Option<String>,
    pub departure_airport: Option<String>,
    pub arrival_airport: Option<String>,
    pub airline_name: Option<String>,
    pub flight_number: Option<String>,
}