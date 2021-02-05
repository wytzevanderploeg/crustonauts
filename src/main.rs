#![feature(proc_macro_hygiene, decl_macro)]
// #! -> Inner attribute, adds compile instructions for this crate in this case

// # -> Outer attribute, define the use of macros
#[macro_use]
extern crate rocket;

// imports for some packages
use rocket::fairing::AdHoc;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Arc;
use std::time::SystemTime;

// Derive macro so Serde knows how to Serialize this struct
#[derive(Serialize)]
struct HelloCrustaceansResponse {
    message: String
}

// Get macro so Rocket knows how to handle this
#[get("/")]
fn hello_crustaceans() -> Json<HelloCrustaceansResponse> {
    // "xyz" is a &str (string-slice).
    Json(HelloCrustaceansResponse { message: "Hello Crustaceans!".to_string() })
}

// Ready for launch!
fn main() {
    let arc = Arc::new(SystemTime::now());

    rocket::ignite()
        .mount("/", routes![hello_crustaceans])
        .attach(AdHoc::on_launch("Startup Time", move |_| {
            let time = arc.clone();
            let elapsed_millis = time.elapsed().unwrap().as_millis();
            println!("Startup Time: {}", elapsed_millis);
        }))
        .launch();
}
