// crate::module;
use uuid::Uuid;


use serde::{Serialize, Deserialize};

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
    pub timestamp: String,
}
// impl Booking {
//     pub fn new(data:&Booking)->Self{
//         let my_uuid = Uuid::new_v4() ;
//         Self{
//             flight_number:data.flight_number.clone(),
//             email:data.email.clone(),
//             passenger_name:data.passenger_name.clone(),
//             id:my_uuid.to_string()
//         }
//     }
// }

pub struct FlightQueryParams {
    pub departure_city: String,
    pub destination_city: String,
    pub date: String,
    pub page: String,
    pub limit:String
}



#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct FlightIdData {
    pub flight_id:String,
    pub passenger_name: String,
    pub email: String,
}