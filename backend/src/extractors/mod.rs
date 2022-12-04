use chrono::NaiveDate;

use crate::models::Event;

mod date_extraction;
pub mod largest_element_with_single_date;

pub trait EventExtractor {
    fn html_to_events(html: &str, from_date: &NaiveDate) -> Vec<Event>;
}
