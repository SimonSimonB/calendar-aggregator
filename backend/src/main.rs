use std::{env, collections::HashMap};
use default_extractor::extractors::{Event};
use rocket::serde::json::Json;
use serde::Deserialize;
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
struct EventsRequest {
    urls: Vec<String>
}

#[post("/events", format = "application/json", data = "<request>")]
async fn get_events(request: Json<EventsRequest>) -> Json<HashMap<String, Vec<Event>>> {
    let urls = &request.urls;
    let mut events: HashMap<String, Vec<Event>> = HashMap::new();
    for url in urls {
        let events_for_url = default_extractor::extract(&url).await.unwrap();
        events.insert(url.to_owned(), events_for_url);
    }

    Json(events)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_events])
        .mount("/", FileServer::from(env::var("FRONTEND_PATH").unwrap()))
}
