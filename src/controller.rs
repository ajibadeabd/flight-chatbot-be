use chrono::{format, Utc, Duration};
use reqwest::Client;
use rocket::{State, serde::json::Json};

use crate::{module::{route_structure::{ApiResponse, FlightQueryParams, FlightData, Booking}, response_handler::CustomError}, model::AppState,  };
use serde_json::from_str;

use uuid::Uuid;





pub async  fn get_flight_schedule(
    db:&State<AppState>,
     flight_query_params: FlightQueryParams)
->Result<ApiResponse, CustomError>
{
 
   let  flight_data = fetch_flight_data(db,Some(flight_query_params.limit)).await?;
   let filtered_flights: Vec<FlightData> = flight_data
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

async  fn fetch_flight_data(db:&State<AppState>,limit:Option<String>)
->Result<Vec<FlightData>, CustomError>
{
    let mut flight_data: Vec<FlightData>  = db.flight_data_db.lock().unwrap().data.clone();
    let mut cache_date  = db.flight_data_db.lock().unwrap().cache_date.clone();
   // println!("{:?}",booking_data);
    println!("{:?}{:?}", flight_data.is_empty() , cache_date < Utc::now());
    let data:Vec<FlightData>;
        if flight_data.is_empty() || cache_date < Utc::now() {
        // make api call
    let client = Client::new();
    let  mut url = "http://api.aviationstack.com/v1/flights?access_key=99951e2bd5da8ce77ad3ab3fdf3209d6".to_owned();
    url = url  + &format!("&limit={}", &limit.unwrap_or("".to_string()));
    println!("{:?}",url);

    // flight_date
    let response_data = client.get(url)
    .send().await.expect("Error calling flight service");
    if response_data.status().is_success() == false {
    // let response  = response_data.text().await.unwrap();
 
            return Err(CustomError::BadRequest("Flight schedule services not available at the moment. please try again".to_owned()));
    }
    let response  = response_data.text().await.unwrap();
 
    let initialize_response:ApiResponse = from_str(&response).unwrap();
   
        db.flight_data_db.lock().unwrap().data= initialize_response.data.clone();
        db.flight_data_db.lock().unwrap().cache_date= Utc::now() + Duration::minutes(10);
       data = initialize_response.data.clone();
    }else{
    data =  flight_data ;
}
Ok(data)

}
pub async  fn book_flight(
    db:&State<AppState>,
     data:Json<Booking>
    )
 ->Result<(), CustomError>
{
   let  flight_data = fetch_flight_data(db,Some("100".to_owned())).await?;

   let is_flight_available = flight_data.iter()
   .find(|&each_flight_data| each_flight_data.flight.number.as_ref()  == Some(&data.flight_number) );
    match is_flight_available {
        Some(dat)=>{

            let new_booking = Booking::new(&data);
                
            println!("{:?}",new_booking);
           Ok( ())

        },
     None=>return Err(CustomError::BadRequest("Flight not found".to_owned()))
            
    }


}