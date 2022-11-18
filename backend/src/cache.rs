extern crate redis;
use redis::Commands;
use redis::Connection;
use rocket::serde::json::serde_json;
use crate::extractors::Event;

pub trait Cache {
    fn set(&mut self, url: &str, events: Vec<Event>);
    fn get(&mut self, url: &str) -> Option<Vec<Event>>;
}

pub struct RedisCache {
    connection: Connection,
}

impl RedisCache {
    pub fn new() -> Result<RedisCache, Box<dyn std::error::Error>> {
        let client = redis::Client::open("redis://redis/")?;
        let connection = client.get_connection()?;
        Ok(RedisCache {
            connection: connection,
        })
    }
}

impl Cache for RedisCache {
    fn set(&mut self, url: &str, events: Vec<Event>) {
        let events_serialized: String = serde_json::to_string(&events).unwrap();
        match self
            .connection
            .set::<&str, &str, String>(url, &events_serialized)
        {
            Err(e) => {
                println!("Failed to set key: {}", e);
            }
            Ok(_) => {
                println!("Successfully set key {}", url);
            }
        }
    }

    fn get(&mut self, url: &str) -> Option<Vec<Event>> {
        match self.connection.get::<&str, String>(url) {
            Ok(events_serialized) => {
                println!("Cache hit for key {}\nValue: {:?}", url, events_serialized);
                Some(serde_json::from_str::<Vec<Event>>(&events_serialized).unwrap())
            },
            Err(e) => {
                println!("Error while querying key {}: {}", url, e);
                None
            },
        }
    }
}
