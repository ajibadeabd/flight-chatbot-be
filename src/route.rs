
use rocket::{ State, serde::json::Json};
 

use crate::{module::{response_handler::{generic_response, CustomError, CustomResult}, route_structure::{FlightQueryParams, FlightIdData, PaymentCallbackUrl, BookingResponse, FlightOption}}, controller, model::AppState};


use rocket::response::content::RawHtml;

#[get("/flights?<params..>")]
pub async fn get_flight_schedule(
        db:&State<AppState>,
        
        params: FlightQueryParams

) -> Result<CustomResult, CustomError>{
    let response: Vec<FlightOption>  = controller::get_flight_schedule(
        db,
        params,
    ).await?;
    Ok(generic_response::<Vec<FlightOption>>(
            "Data retrieved successfully.",
           Some(response),
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
    Ok(generic_response::<BookingResponse>(
            format!("Booking confirmed for flight {}. Please proceed to payment.",booking_id).as_str(),
           Some(BookingResponse{
            booking_id:booking_id
           }),
           None
       ))
}


#[get("/initialize_payment/<booking_id>", )]
pub async fn payment_initiate(
        db:&State<AppState>,
        booking_id:String,

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
 ->RawHtml<String>
{ 
    RawHtml(controller::get_payment_page(db,payment_id).await)
}

#[post("/payment",data = "<data>" )]
pub async fn post_payment_page(
        db:&State<AppState>,
        data:Json<PaymentCallbackUrl>
) -> Result<CustomResult, CustomError>{
       let _response =  controller::make_payment_page(db, &data).await?;
       Ok(generic_response::<String>(
         "Payment made successfully.",
       None,
       None
   ))
}

