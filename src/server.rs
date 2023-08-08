#[macro_use]
extern crate rocket;
mod model;
mod module;
mod controller;
mod route;

use crate::route::{
    get_flight_schedule,
    flight_option,
    booking,
    payment_initiate,
    get_payment_page
};


#[launch]
fn rocket() -> _ {
let app_data = model::AppState::init();
    let rocket = rocket::build()
    .mount("/api", routes![
        get_flight_schedule,
        flight_option,
        booking,
        payment_initiate,
        get_payment_page
        ]).manage(app_data) ;
     rocket
}

