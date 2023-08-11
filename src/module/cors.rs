use rocket_cors::{
    CorsOptions, AllowedHeaders, AllowedOrigins,
    Cors as RocketCors
};
 
use std::str::FromStr;
use rocket_cors::AllowedMethods;
 
 

pub fn make_cors() -> RocketCors {
    let allowed_origins = AllowedOrigins::all();
    // let allowed_origins = AllowedOrigins::some_exact(&[
    //     "http://localhost:3000",
    //     "http://127.0.0.1:8080",
    //     "http://localhost:8000",
    //     "http://0.0.0.0:8000",
    // ]);
    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();
    let cors_options = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        // allowed_headers: AllowedHeaders::some(&[
        //     "Authorization",
        //     "Accept",
        //     "Access-Control-Allow-Origin",
        // ]),
        allow_credentials: true,
        ..Default::default()
    };
    RocketCors::from_options(&cors_options).expect("Error building CORS fairing")
}