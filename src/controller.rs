
use chrono::{ Utc};
use rocket::{State, serde::json::Json};

use crate::{module::{route_structure::{ FlightQueryParams, FlightData, Booking, FlightIdData, Payment, TransactionType, PaymentCallbackUrl, FlightOption}, response_handler::CustomError}, model::AppState,  };

use uuid::Uuid;

 


pub async  fn get_flight_schedule(
    db:&State<AppState>,
     flight_query_params: FlightQueryParams)
->Result<Vec<FlightOption>, CustomError>
{
    let flight_data: Vec<FlightData>  = db.flight_data_db.lock().unwrap().data.clone();
    let skip = (flight_query_params.page.unwrap_or(1) - 1) * flight_query_params.limit.unwrap_or(50);

    
    let filtered_flights: Vec<FlightOption> = flight_data
    .iter()
    .skip(skip as usize)
    .take(flight_query_params.limit.unwrap_or(20) as usize )
    .filter(|flight| {
      
      
        let departure_airport = &flight.departure.airport;
        let destination_airport = &flight.arrival.airport;
        let flight_date = flight.flight_date.as_ref(); // Borrow the option content
     
     
     let mut allTrue :bool=true;
     
     if flight_query_params.departure_city.is_some() {
        if !allTrue {
            return allTrue
        }
        let main_field =&departure_airport.to_owned().unwrap().to_lowercase();
        let search_field =&flight_query_params.departure_city.to_owned().unwrap().to_lowercase();
        allTrue = main_field.find(search_field).is_some();
     }

     if flight_query_params.destination_city.is_some() {
        if !allTrue {
            return allTrue
        }
        let main_field =&destination_airport.to_owned().unwrap().to_lowercase();
        let search_field =&flight_query_params.destination_city.to_owned().unwrap().to_lowercase();
        allTrue = main_field.find(search_field).is_some();
     }



     if flight_query_params.date.is_some() {
        if !allTrue {
            return allTrue
        }
        let main_field =&flight_date.to_owned().unwrap().to_lowercase();
        let search_field =&flight_query_params.date.to_owned().unwrap().to_lowercase();
        allTrue = main_field.find(search_field).is_some();
     }

     return allTrue;
    })
    .map(|each_data| {
        FlightOption {
    flight_date: each_data.flight_date.to_owned(),
    scheduled_departure: each_data.departure.scheduled.to_owned(),
    scheduled_arrival: each_data.arrival.scheduled.to_owned(),
    departure_airport: each_data.departure.airport.to_owned(),
    arrival_airport: each_data.arrival.airport.to_owned(),
    airline_name:each_data.airline.name.to_owned(),
    flight_number:each_data.flight.number.to_owned(),
        }
        
    }).into_iter().collect();
   Ok(filtered_flights)

}


 
 
pub async  fn booking(
    db:&State<AppState>,
    payload:Json<FlightIdData>
    )
 ->Result<String, CustomError>
{
let flight_data: Vec<FlightData>  = db.flight_data_db.lock().unwrap().data.clone();

   let is_flight_available = flight_data.iter()
   .find(|&each_flight_data| each_flight_data.flight.number.as_ref()  == Some(&payload.flight_id) );
    match is_flight_available {
        Some(dat)=>{
            let booking_id = Uuid::new_v4().to_string();
            let new_booking = Booking {
                flight_number:payload.flight_id.to_owned(),
                email:payload.email.to_owned(),
                passenger_name:payload.passenger_name.to_owned(),
                id:booking_id.clone(),
                payment_details:None,
                amount:700.10
            };
            db.booking_db.lock().unwrap().push(new_booking);
           Ok(booking_id)

        },
     None=>return Err(CustomError::BadRequest("Flight details  not found".to_owned()))
            
    }


}
pub async  fn payment_initiate(
    db:&State<AppState>,
    booking_id:String
    )
 ->Result<String, CustomError>
{

    let valid_booking = db.booking_db.lock().unwrap().iter().find(
        |each_booking| each_booking.id==booking_id).cloned();

    match &valid_booking {
        Some(data)=>{
            let payment_id = Uuid::new_v4().to_string();


            let new_payment = Payment {
                email:data.email.to_owned(),
                currency:String::from("USD"),
                id:payment_id,
                status:TransactionType::PENDING,
                timestamp:Utc::now().to_string(),
                amount:data.amount,
                transaction_reference:data.id.to_owned()
            };
            let mut  payment_link = format!("http://localhost:8000/payment/{}",new_payment.id);
            let duplicate_transaction = db.payment_db.lock().unwrap().iter().find(
                |payment|{
                     payment.transaction_reference == new_payment.transaction_reference &&
                     payment.status==TransactionType::PENDING
                    }
            ).cloned();
            if let Some(duplicate_payment) =  duplicate_transaction {
                payment_link  = format!("http://localhost:8000/payment/{}",duplicate_payment.id);
               return Ok(payment_link)            
            }
            db.payment_db.lock().unwrap().push(new_payment);
            Ok(payment_link)            
        },
        _=>return Err(CustomError::BadRequest("Invalid booking id.".to_owned()))
    }


}


pub async  fn get_payment_page(
    db:&State<AppState>,
    payment_id:String
    )
->String
{
    let valid_payment = db.payment_db.lock().unwrap().iter().find(
        |payment_data| {
            payment_data.id==payment_id 
            &&
            payment_data.status==TransactionType::PENDING
        }).cloned(); 
        if let None = &valid_payment {
            let response_body = "<html><body><h1>Invalid payment link.</h1></body></html>".to_owned();
        return response_body;
        }
    return generate_payment_page(150.00, "USD",valid_payment.unwrap().id)

}
pub async  fn make_payment_page(
    db:&State<AppState>,
   data:&Json<PaymentCallbackUrl>
    )
 ->Result<(), CustomError>
{
    let valid_payment = db.payment_db.lock().unwrap().iter().find(
        |payment_data| payment_data.id==data.payment_id).cloned(); 
        
        if let None = &valid_payment {
        return Err(CustomError::BadRequest("Invalid payment id".to_string()))
        }

        if let TransactionType::SUCCESS = &valid_payment.unwrap().status {
            return Err(CustomError::BadRequest("payment has been completed already".to_string()))
            }
        let mut payment_db = db.payment_db.lock().unwrap();
        
        for payment in payment_db.iter_mut() {
        if payment.id == data.payment_id {
            payment.status = TransactionType::SUCCESS;
            break; 
        }
    }
        Ok(())
}



fn  generate_payment_page(payment_amount: f32, currency: &str,payment_id:String) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Payment Page</title>
        </head>
        <body>

            <h1>Payment id : {}</h1>
            <h2>Payment Amount: {} {}</h2>
            <button id="make_payment">Make Payment</button>


            <script>
            
                const makePaymentButton = document.getElementById("make_payment");
                makePaymentButton.addEventListener("click", async () => {{
                    try {{
                        const callbackUrl = new URLSearchParams(window.location.search).get('callback_url');
                        const paymentId = window.location.pathname.split("/")[2];
                      


                        // Display loading message
                        const loadingMessage = document.createElement("p");
                        loadingMessage.textContent = "Loading...";
                        document.body.appendChild(loadingMessage);

                        function delay(ms) {{
                            return new Promise(resolve => setTimeout(resolve, ms));
                          }}

                          await delay(2000)


                        // let block = new Promise(resolve => setTimeout(resolve, 2000));

                        // Call your API endpoint here to process the payment



                        const response = await fetch("/api/payment", {{
                            method: "POST",
                            
                            body: JSON.stringify({{
                                payment_id: paymentId,  
                            }}),
                        }});
                        if (response.ok) {{
                        loadingMessage.textContent = "";

                            // Payment was successful, redirect to the callback URL
                            const successMessage = document.createElement("h2");
                            successMessage.textContent = "Payment Successfully as you will be redirected to you flight page";
                            successMessage.style.color = "green";
                            document.body.appendChild(successMessage);
                    
                            // Redirect to the callback URL after 5 seconds
                            setTimeout(() => {{
                              window.location.href = callbackUrl + "?reference=" + paymentId;
                            }}, 3000);




                        }} else {{
                            const errorData = await response.json();
                            console.log("Payment error:", errorData);
                            alert("Payment failed. Please try again.");
                        }}
                        // Handle the response as needed
                    }} catch (error) {{
                        alert(error?.message)
                        console.log("Error making payment:", error);
                    }}
                }});
            </script>
        </body>
        </html>
        "#,
        payment_id,payment_amount, currency
    )
}
