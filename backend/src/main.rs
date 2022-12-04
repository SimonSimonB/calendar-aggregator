use default_extractor::cache::{Cache, RedisCache};
use default_extractor::models::{Event, Topic};
use default_extractor::repository::Repository;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;
use std::{collections::HashMap, env};

#[macro_use]
extern crate rocket;

async fn get_event_json_for_urls(urls: Vec<String>) -> Json<HashMap<String, Vec<Event>>> {
    let mut cache = RedisCache::new().unwrap();
    let mut events: HashMap<String, Vec<Event>> = HashMap::new();
    for url in urls {
        let events_for_url = match cache.get(&url) {
            None => {
                println!("Extract events for {}...", url);
                let extracted_events = default_extractor::extract(&url).await.unwrap();
                println!("Found {} events for {}.", extracted_events.len(), url);
                cache.set(&url, extracted_events.clone());
                extracted_events
            }
            Some(cached_events) => cached_events,
        };

        events.insert(url.to_owned(), events_for_url);
    }

    Json(events)
}

#[get("/events?<topic_id>", format = "application/json", rank = 2)]
async fn get_events_by_topic(
    topic_id: u32,
    repository: &State<Repository>,
) -> Json<HashMap<String, Vec<Event>>> {
    let topic_urls = repository.get_topic_urls(topic_id).await.unwrap();
    get_event_json_for_urls(topic_urls).await
}

#[get("/topics", format = "application/json")]
async fn get_topics(
    repository: &State<Repository>,
) -> Json<Vec<Topic>> {
    // Just for debugging
    if repository.get_topics().await.unwrap().len() == 0 {
        repository.add_topic("Essen", vec![String::from("https://ruhrmuseum.de/veranstaltungen/kalender")]).await.unwrap();
    }
    let topics = repository.get_topics().await.unwrap();
    Json(topics)
}

#[get("/events?<url>", format = "application/json", rank = 3)]
async fn get_events_by_url(url: String) -> Json<HashMap<String, Vec<Event>>> {
    get_event_json_for_urls(vec![url]).await
}

#[launch]
async fn rocket() -> _ {
    let repository = Repository::new().await.unwrap();
    rocket::build()
        .manage(repository)
        .mount("/api", routes![get_events_by_topic, get_events_by_url, get_topics])
        .mount("/", FileServer::from(env::var("FRONTEND_PATH").unwrap()))
}
