use std::convert::Infallible;

use rocket::{data::Limits, State, serde::json::Json, http::hyper::{Response, Body}};

use crate::{module::{response_handler::{generic_response, CustomError, CustomResult}, route_structure::{ApiResponse, FlightQueryParams, Booking, FlightIdData}}, controller, model::AppState};



use rocket::response::content::RawHtml;


#[get("/flights?<destination_city>&<departure_city>&<date>&<limit>&<page>" )]
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

#[post("/flight/option" )]
pub async fn flight_option(
        db:&State<AppState>,
) -> Result<CustomResult, CustomError>{
    let response  = controller::flight_option(
        db,
        // data
    ).await?;
    Ok(generic_response::<ApiResponse>(
            "Data retrieved successfully.",
           None,
        //    Some(response),
           None
       ))
}


#[post("/booking", data="<payload>" )]
pub async fn booking(
        db:&State<AppState>,
        payload:Json<FlightIdData>
) -> Result<CustomResult, CustomError>{
    let booking_id  = controller::booking(
        db,
        payload
    ).await?;
    Ok(generic_response::<ApiResponse>(
            format!("Booking confirmed for flight {}. Please proceed to payment.",booking_id).as_str(),
           None,
           None
       ))
}


#[get("/initialize_payment/<booking_id>", )]
pub async fn payment_initiate(
        db:&State<AppState>,
        booking_id:String
) -> Result<CustomResult, CustomError>{
    let payment_link  = controller::payment_initiate(
        db,
        booking_id
    ).await?;
    Ok(generic_response::<String>(
            "Payment link initialized successfully",
           Some(payment_link),
           None
       ))
}

#[get("/payment/<payment_id>", )]
pub async fn get_payment_page(
        db:&State<AppState>,
        payment_id:String)
 ->RawHtml<&str>
{ 
       let response = controller::get_payment_page(db).await;
RawHtml(&response) 
}

