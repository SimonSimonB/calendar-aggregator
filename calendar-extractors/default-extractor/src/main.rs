use std::env;
use default_extractor::Event;
use rocket::serde::json::Json;
use serde::Deserialize;
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
struct EventsRequest {
    urls: Vec<String>
}

#[post("/events", format = "application/json", data = "<request>")]
async fn get_events(request: Json<EventsRequest>) -> Json<Vec<Event>> {
    let urls = &request.urls;
    let events = default_extractor::extract(&urls[0]).await.unwrap();

    Json(events)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_events])
        .mount("/", FileServer::from(env::var("FRONTEND_PATH").unwrap()))
}
