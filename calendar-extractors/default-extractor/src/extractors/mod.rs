use std::fmt;

use chrono::{NaiveDateTime, NaiveDate};
use serde::ser::{Serializer, SerializeStructVariant};

pub mod largest_element_with_single_date;
mod date_extraction;

pub trait EventExtractor {
  fn code_to_events(website_code: &str, from_date: &NaiveDate) -> Vec<Event>;
}

#[derive(Debug, Copy, Clone)]
pub enum NaiveDateWithOptionalTime {
  NaiveDate(NaiveDate),
  NaiveDateTime(NaiveDateTime),
}

impl serde::Serialize for NaiveDateWithOptionalTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    match self {
      NaiveDateWithOptionalTime::NaiveDate(d) => {
        let mut sv = serializer.serialize_struct_variant("E", 0, "NaiveDate", 1)?;
        sv.serialize_field("date", &d.to_string())?;
        sv.end()
      },
      NaiveDateWithOptionalTime::NaiveDateTime(d) => {
        let mut sv = serializer.serialize_struct_variant("E", 0, "NaiveDateTime", 1)?;
        sv.serialize_field("date", &d.to_string())?;
        sv.end()
      },
    }
  }
}

impl From<NaiveDate> for NaiveDateWithOptionalTime {
  fn from(date: NaiveDate) -> Self {
    NaiveDateWithOptionalTime::NaiveDate(date)
  }
}

#[derive(Debug, serde::Serialize)]
pub struct Event {
  pub text: String,
  pub time: NaiveDateWithOptionalTime,
}

impl fmt::Display for Event {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.text, self.time)
  }
}

impl fmt::Display for NaiveDateWithOptionalTime {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Event {
  pub fn new(text: &str, time: NaiveDateWithOptionalTime) -> Event {
    Event { 
      text: text.to_owned(),
      time: time,
    }
  } 
}