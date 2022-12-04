use chrono::NaiveDate;
use std::fmt;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Topic {
  pub id: u32,
  pub name: String,
  pub urls: Vec<String>,
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
