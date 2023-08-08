#[macro_use]
extern crate rocket;
mod model;
mod module;
mod controller;
mod route;
use crate::route::{get_flight_schedule,book_flight};


#[launch]
fn rocket() -> _ {
let app_data = model::AppState::init();
    let rocket = rocket::build()
    .mount("/api", routes![get_flight_schedule,book_flight]).manage(app_data) ;
     rocket
}

