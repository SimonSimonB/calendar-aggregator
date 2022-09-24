use std::{env, collections::HashMap, sync::Mutex};
use chrono::{Utc, DateTime};
use default_extractor::extractors::{Event};
use rocket::{serde::json::Json, State};
use serde::Deserialize;
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

struct Cache(Mutex<HashMap<String, CacheValue>>);

struct CacheValue {
    events: Vec<Event>,
    time: DateTime<Utc>,
}

#[derive(Deserialize)]
struct EventsRequest {
    urls: Vec<String>
}

fn read_from_cache(cache_mutex: &Mutex<HashMap<String, CacheValue>>, url: &str) -> Option<Vec<Event>> {
    let cache = cache_mutex.lock().expect("cannot get cache");
    if cache.contains_key(url) && (Utc::now() - cache[url].time) < chrono::Duration::minutes(5) {
        return Some(cache[url].events.to_vec());
    }
    else {
        return None;
    }
}

fn write_to_cache(cache_mutex: &Mutex<HashMap<String, CacheValue>>, url: &str, events: Vec<Event>) {
    let mut cache = cache_mutex.lock().expect("cannot get cache");
    cache.insert(url.to_string(), CacheValue { events: events, time: Utc::now()});
}

#[post("/events", format = "application/json", data = "<request>")]
async fn get_events(request: Json<EventsRequest>, cache_mutex: &State<Cache>) -> Json<HashMap<String, Vec<Event>>> {
    let urls = &request.urls;
    let mut events: HashMap<String, Vec<Event>> = HashMap::new();
    for url in urls {
        let cached_result = read_from_cache(&cache_mutex.0, url);
        let events_for_url = match cached_result {
            None => default_extractor::extract(&url).await.unwrap(),
            Some(v) => v
        };

        write_to_cache(&cache_mutex.0, url, events_for_url.to_vec());

        events.insert(url.to_owned(), events_for_url);
    }

    Json(events)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_events])
        .mount("/", FileServer::from(env::var("FRONTEND_PATH").unwrap()))
        .manage(Cache(Mutex::new(HashMap::new())))
}
