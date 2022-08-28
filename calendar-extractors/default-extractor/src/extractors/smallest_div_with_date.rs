use regex::Regex;
use scraper::{ElementRef};

use super::Event;
use super::EventExtractor;
use super::date_extraction;

pub struct SmallestDivWithDateExtractor {
}


impl EventExtractor for SmallestDivWithDateExtractor {
  fn code_to_events(website_code: &str) -> Vec<Event> {
    let document = scraper::Html::parse_document(website_code);
    let start_elements_to_try = ["main", "body", "html"];
    for start_element in start_elements_to_try {
      let main_selector = scraper::Selector::parse(start_element).unwrap();
      let start_elements: Vec<ElementRef> = document.select(&main_selector).collect::<Vec<ElementRef>>();
      if start_elements.len() > 0 {
        return element_to_events(&start_elements[0])
      }
    }
    // TODO: Could return an error here instead of returning an empty list. That might help with debugging empty results
    // in production. Or, at least, log here that no main element was found.
    return Vec::new();
  }
}

fn element_to_events(el: &ElementRef) -> Vec<Event> {
  let element_kinds_to_traverse = ["div", "section", "ul", "li"];
  let mut events = Vec::new();
  for child in el.children() {
    if child.value().is_element() {
      let child_element = child.value().as_element().unwrap();
      if element_kinds_to_traverse.contains(&child_element.name()) {
        let events_from_child = element_to_events(&ElementRef::wrap(child).unwrap());
        if events_from_child.len() > 0 {
          events.extend(events_from_child);
        }
      }
    }
  }

  // If, and only if, no events were found in each of the children, then try to extract an event from the totality of 
  // text in this element.
  if events.is_empty() {
    if let Some(event) = div_to_one_event(el) {
      events.push(event);
    }
  }

  events
}

fn div_to_one_event(el: &ElementRef) -> Option<Event> {
  let dates = date_extraction::extract_datetimes(el);

  if dates.len() != 1 {
    return None
  }
  let date = &dates[0];

  let text = extract_text(el, &date.matched_string);
  if text.len() <= 15 {
    return None
  }

  Some(Event::new(&text, date.date, None))
}

fn extract_text(div: &ElementRef, date_text: &str) -> String {
  let all_text: String = div.text().collect::<String>();
  // Replace all sequences of whitespaces with a single white space
  let mut text = Regex::new(r"\s+").unwrap().replace_all(&all_text, " ").to_string();
  text = text.replace(date_text, "");
  return text;
}