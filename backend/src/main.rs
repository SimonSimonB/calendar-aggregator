use default_extractor::cache::{Cache, RedisCache};
use default_extractor::extractors::Event;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::{collections::HashMap, env};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct EventsRequest {
    urls: Vec<String>,
}

#[post("/events", format = "application/json", data = "<request>")]
async fn get_events(request: Json<EventsRequest>) -> Json<HashMap<String, Vec<Event>>> {
    let mut cache = RedisCache::new().unwrap();
    let urls = &request.urls;
    let mut events: HashMap<String, Vec<Event>> = HashMap::new();
    for url in urls {
        let events_for_url = match cache.get(url) {
            None => {
                let extracted_events = default_extractor::extract(&url).await.unwrap();
                cache.set(url, extracted_events.clone());
                extracted_events
            },
            Some(cached_events) => cached_events,
        };

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
