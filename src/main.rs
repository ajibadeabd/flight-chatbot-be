
#[macro_use]
extern crate rocket;

mod model;
mod module;
mod controller;
mod route;

use crate::route::{
    get_flight_schedule,
    booking,
    payment_initiate,
    get_payment_page,
    post_payment_page
};
use crate::module::error_handler::{
    not_found,
    internal_error

};
use module::cors::make_cors;



//#[main]
// #[launch]

 #[shuttle_runtime::main]
pub async fn rocket() ->  shuttle_rocket::ShuttleRocket {
//    pub async fn rocket()  -> _ {
let app_data = model::AppState::init();
    let rocket = rocket::build()
    .mount("/api", routes![
        get_flight_schedule,
        booking,
        payment_initiate,
        post_payment_page
        ])
   .register("/", catchers![
           not_found,
            internal_error,
            rocket_validation::validation_catcher
            ]) 
    .mount(
                "/",routes![get_payment_page]
            ).manage(app_data) .attach(make_cors()).into();
    // rocket
     Ok(rocket)
}

// // // Unit testings
// // #[cfg(test)]
// // mod test; 


 