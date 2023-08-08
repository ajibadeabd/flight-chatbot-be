use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::module::route_structure::{Booking, FlightData, Payment};

#[derive(Debug, Deserialize, Serialize,Clone)]

pub struct DbFlightData {
    pub data: Vec<FlightData>,
    pub cache_date: DateTime<Utc>,
}
pub struct AppState {
    pub  booking_db: Arc<Mutex<Vec<Booking>>>,
    pub  payment_db: Arc<Mutex<Vec<Payment>>>,
    pub  flight_data_db: Arc<Mutex<DbFlightData>>,
}



impl AppState {
    pub fn init() -> AppState {
        AppState {
            booking_db: Arc::new(Mutex::new(Vec::new())),
            payment_db: Arc::new(Mutex::new(Vec::new())),
            flight_data_db: Arc::new(Mutex::new(DbFlightData {
                data: Vec::new(),
                cache_date: Utc::now(), // Set the initial cache_date to the current UTC time
            })),
        }
    }
}
