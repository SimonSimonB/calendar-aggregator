use chrono::NaiveDate;
use regex::Regex;
use scraper::{ElementRef};
use super::Event;

use super::EventExtractor;
use super::NaiveDateWithOptionalTime;

pub struct SmallestDivWithDateExtractor {
}


impl EventExtractor for SmallestDivWithDateExtractor {
  fn code_to_events(website_code: &str) -> Vec<Event> {
    let document = scraper::Html::parse_document(website_code);
    let main_selector = scraper::Selector::parse("main").unwrap();

    let main_elements: Vec<ElementRef> = document.select(&main_selector).collect::<Vec<ElementRef>>();
    if main_elements.len() == 0 {
      // TODO: Could return an error here instead of returning an empty list. That might help with debugging empty results
      // in production.
      return Vec::new();
    }
    div_to_events(&main_elements[0])
  }
}

fn div_to_events(el: &ElementRef) -> Vec<Event> {
  let element_kinds_to_traverse = ["div", "section"];
  let mut events = Vec::new();
  for child in el.children() {
    if child.value().is_element() {
      let child_element = child.value().as_element().unwrap();
      if element_kinds_to_traverse.contains(&child_element.name()) {
        let events_from_child = div_to_events(&ElementRef::wrap(child).unwrap());
        if events_from_child.len() > 0 {
          events.extend(events_from_child);
        }
      }
    }
  }

  if events.is_empty() {
    if let Some(event) = div_to_one_event(el) {
      events.push(event);
    }
  }

  events
}

fn div_to_one_event(el: &ElementRef) -> Option<Event> {
  let dates = extract_datetimes(el);
  let text = extract_text(el);

  if dates.len() == 1 && text.len() > 15 {
    Some(Event::new(&text, dates[0], None))
  } else {
    None
  }
}

fn extract_datetimes(div: &ElementRef) -> Vec<NaiveDateWithOptionalTime> {
  let text: String = div.text().collect();
  let re = Regex::new(r"(\d{1,2})\.(\d{1,2})\.").unwrap();
  let mut results: Vec<NaiveDateWithOptionalTime> = Vec::new();
  for captured in re.captures_iter(&text) {
    let month = captured[2].parse::<u32>().unwrap();
    let day = captured[1].parse::<u32>().unwrap();
    results.push(NaiveDate::from_ymd(2022, month, day).into())
  }

  return results;
}

fn extract_text(div: &ElementRef) -> String {
  let all_text: String = div.text().collect::<String>();
  // Replace all sequences of whitespaces with a single white space
  Regex::new(r"\s+").unwrap().replace_all(&all_text, " ").to_string()
}
