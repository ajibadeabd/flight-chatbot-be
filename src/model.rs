use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::{module::route_structure::{Booking, FlightData, Payment, ApiResponse}};

use std::fs::File;
use std::io::{Read};
use serde_json::{ Error};
use serde_json::from_str;




#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct DbFlightData {
    pub data: Vec<FlightData>,
    // pub data: Vec<FlightData>,
    pub cache_date: DateTime<Utc>,
}
pub struct AppState {
    pub  booking_db: Arc<Mutex<Vec<Booking>>>,
    pub      payment_db: Arc<Mutex<Vec<Payment>>>,
    // pub  flight_data_db: Arc<Mutex<Vec:>>,
    pub  flight_data_db: Arc<Mutex<DbFlightData>>,
}



impl AppState {
    pub fn init() -> AppState {
        AppState {
            booking_db: Arc::new(Mutex::new(Vec::new())),
            payment_db: Arc::new(Mutex::new(Vec::new())),
            flight_data_db: Arc::new(Mutex::new(DbFlightData {
                data:Vec::new(),
                // data: read_flight_json_file().unwrap(),
                cache_date: Utc::now(), // Set the initial cache_date to the current UTC time
            })),
        }
    }
}

//  fn read_flight_json_file() -> Result<Vec<FlightData>, Error> {
//     let mut file = File::open("flight.json").expect("Unable to open file");
//     // Read the file content into a string
//     let mut content = String::new();
//     file.read_to_string(&mut content).expect("Unable to read file");

//     // Parse the JSON cçontent into FlightEntry struct
//     let flight_entries: ApiResponse = from_str(&content).expect("Unable to parse JSON");
// Ok(flight_entries.data)
// }