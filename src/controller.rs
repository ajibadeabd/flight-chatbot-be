use chrono::{format, Utc, Duration};
use reqwest::Client;
use rocket::State;

use crate::{module::{route_structure::{ApiResponse, FlightQueryParams, FlightData}, response_handler::CustomError}, model::AppState,  };
use serde_json::from_str;






pub async  fn get_flight_schedule(
    db:&State<AppState>,
     flight_query_params: FlightQueryParams)
->Result<ApiResponse, CustomError>
{
    let mut booking_data: Vec<FlightData>  = db.flight_data_db.lock().unwrap().data.clone();
    let mut cache_date  = db.flight_data_db.lock().unwrap().cache_date.clone();
   // println!("{:?}",booking_data);
    println!("{:?}{:?}", booking_data.is_empty() , cache_date < Utc::now());
    let s:ApiResponse;
        if booking_data.is_empty() || cache_date < Utc::now() {
        // make api call
    let client = Client::new();
    let  mut url = "http://api.aviationstack.com/v1/flights?access_key=99951e2bd5da8ce77ad3ab3fdf3209d6".to_owned();
    url = url  + &format!("&limit={}", &flight_query_params.limit);
    println!("{:?}",url);

    // flight_date
    let response_data = client.get(url)
    .send().await.expect("Error calling flight service");
    if response_data.status().is_success() == false {
    let response  = response_data.text().await.unwrap();
    // println!("{:?}",response);
            return Err(CustomError::BadRequest("Flight schedule services not available at the moment. please try again".to_owned()));
    }
    let response  = response_data.text().await.unwrap();
//println!("{:?}",response);
    let initialize_response:ApiResponse = from_str(&response).unwrap();
    
//     let filtered_flights: Vec<FlightData> = initialize_response.data
//     .iter()
//     .filter(|flight| {
//         let departure_airport = &flight.departure.airport;
//         let destination_airport = &flight.arrival.airport;
//         let flight_date = flight.flight_date.as_ref(); // Borrow the option content
     
//      if departure_airport != &Some("".to_owned()) {
//         return departure_airport == &Some(flight_query_params.departure_city.clone())
//      }
//      else if destination_airport != &Some("".to_owned()) {
//         println!("{:?}{:?}",destination_airport , &Some(flight_query_params.destination_city.clone()));
//         return destination_airport == &Some(flight_query_params.destination_city.clone())
//      }else{
// return         flight_date == Some(&flight_query_params.date)

//      }
//     })
    // .cloned() // Use .cloned() to create a new FlightData object
    // .collect();
        db.flight_data_db.lock().unwrap().data= initialize_response.data.clone();
        db.flight_data_db.lock().unwrap().cache_date= Utc::now() + Duration::minutes(10);
       s = initialize_response;
//   return  Ok(ApiResponse { data: filtered_flights })
}else{
    s = ApiResponse {   data: booking_data };
}
// print!("{:?}",s);
let filtered_flights: Vec<FlightData> = s.data
    .iter()
    .filter(|flight| {
        let departure_airport = &flight.departure.airport;
        let destination_airport = &flight.arrival.airport;
        let flight_date = flight.flight_date.as_ref(); // Borrow the option content
     let mut allTrue :bool=true;
     if  &Some(flight_query_params.departure_city.clone()) != &Some("".to_owned()) {
        if !allTrue {
            return allTrue
        }
        allTrue = departure_airport == &Some(flight_query_params.departure_city.clone())
     }
     if &Some(flight_query_params.destination_city.clone()) != &Some("".to_owned()) {
        if !allTrue {
            return allTrue
        }
        allTrue = destination_airport == &Some(flight_query_params.destination_city.clone())
     }
     if  flight_query_params.date  != "".to_owned() {
        if !allTrue {
            return allTrue
        }
        allTrue = flight_date == Some(&flight_query_params.date)

     }
     return allTrue;
    })
    .cloned() // Use .cloned() to create a new FlightData object
    .collect();
// Ok(ApiResponse {   data: booking_data })
   Ok(   ApiResponse {   data:filtered_flights })

}