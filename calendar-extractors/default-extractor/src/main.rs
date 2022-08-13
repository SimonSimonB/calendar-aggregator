use default_extractor::Event;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/")]
async fn get_events() -> Json<Vec<Event>> {
    Json(default_extractor::extract("https://www.theater-essen.de/philharmoniker/spielplan/aaltomusiktheater/aalto-ballett-essen/essener-philharmoniker/philharmonie-essen/").await.unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/events", routes![get_events])
}
