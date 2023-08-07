use rocket::{data::Limits, State};

use crate::{module::{response_handler::{generic_response, CustomError, CustomResult}, route_structure::{ApiResponse, FlightQueryParams}}, controller, model::AppState};



 

#[get("/flight_schedule?<destination_city>&<departure_city>&<date>&<limit>&<page>" )]
pub async fn get_flight_schedule(
        db:&State<AppState>,
        departure_city:Option<String>,
        destination_city:Option<String>,
        date:Option<String>,
        limit:Option<i32>,
        page:Option<i32>,

) -> Result<CustomResult, CustomError>{
    let response  = controller::get_flight_schedule(
        db,
        FlightQueryParams{
                departure_city:departure_city.unwrap_or("".to_owned()),
                destination_city:  destination_city.unwrap_or("".to_owned()),
                date:date.unwrap_or("".to_owned()),
                page:page.unwrap_or(1).to_string(),
                limit:limit.unwrap_or(10).to_string(),
        }
    ).await?;
    Ok(generic_response::<ApiResponse>(
            "Data retrieved successfully.",
           Some(response),
           None
       ))
}