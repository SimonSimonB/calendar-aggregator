use std::fmt;

use chrono::{NaiveDate};

mod date_extraction;
pub mod largest_element_with_single_date;

pub trait EventExtractor {
    fn code_to_events(website_code: &str, from_date: &NaiveDate) -> Vec<Event>;
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub text: String,
    pub date: NaiveDate,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.text, self.date)
    }
}

impl Event {
    pub fn new(text: &str, date: NaiveDate) -> Event {
        Event {
            text: text.to_owned(),
            date,
        }
    }
}
