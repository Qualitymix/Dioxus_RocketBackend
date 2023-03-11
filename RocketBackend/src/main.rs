#[macro_use]
extern crate rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions, Guard};

fn cors_options_all() -> CorsOptions {
    Default::default()
}

// fn cors_options() -> CorsOptions {
//     let allowed_origins = AllowedOrigins::some_exact(&["localhost:8080"]);
//     rocket_cors::CorsOptions {
//         allowed_origins,
//         allowed_methods: vec![rocket::http::Method::Get]
//             .into_iter()
//             .map(From::from)
//             .collect(),
//         allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
//         allow_credentials: true,
//         ..Default::default()
//     }
// }

#[get("/")]
fn index(cors: Guard<'_>) -> rocket_cors::Responder<&str> {
    // Uncomment the thread sleep in order to simulate time taken to load from backend.
    // std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    cors.responder("Hello from Rocket!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", rocket_cors::catch_all_options_routes())
        .manage(cors_options_all().to_cors().expect("to not fail"))
}
