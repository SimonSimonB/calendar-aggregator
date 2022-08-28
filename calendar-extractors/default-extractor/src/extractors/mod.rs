use std::fmt;

use chrono::{NaiveDateTime, NaiveDate};
use serde::ser::{Serializer, SerializeStructVariant};

pub mod smallest_div_with_date;
mod date_extraction;

pub trait EventExtractor {
  fn code_to_events(website_code: &str) -> Vec<Event>;
}

#[derive(Debug, serde::Serialize)]
pub struct EventTimeRange {
  pub start: NaiveDateWithOptionalTime,
  pub end: Option<NaiveDateWithOptionalTime>,
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
  pub time: EventTimeRange,
}

impl fmt::Display for Event {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.text, self.time)
  }
}

impl fmt::Display for EventTimeRange {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.end {
      Some(end) => write!(f, "{} {}", self.start, end),
      None => write!(f, "{}", self.start),
    }
  }
}

impl fmt::Display for NaiveDateWithOptionalTime {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Event {
  pub fn new(text: &str, start: NaiveDateWithOptionalTime, end: Option<NaiveDateWithOptionalTime>) -> Event {
    Event { 
      text: text.to_owned(),
      time: EventTimeRange { start: start, end: end },
    }
  } 
}