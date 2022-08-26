use std::collections::HashMap;

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
    let start_elements_to_try = ["main", "body", "html"];
    for start_element in start_elements_to_try {
      let main_selector = scraper::Selector::parse(start_element).unwrap();
      let start_elements: Vec<ElementRef> = document.select(&main_selector).collect::<Vec<ElementRef>>();
      if start_elements.len() > 0 {
        return div_to_events(&start_elements[0])
      }
    }
    // TODO: Could return an error here instead of returning an empty list. That might help with debugging empty results
    // in production. Or, at least, log here that no main element was found.
    return Vec::new();
  }
}

fn div_to_events(el: &ElementRef) -> Vec<Event> {
  let element_kinds_to_traverse = ["div", "section", "ul", "li"];
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
  let text = extract_text(el);
  let dates = extract_datetimes(&text);

  if dates.len() == 1 && text.len() > 15 {
    Some(Event::new(&text, dates[0], None))
  } else {
    None
  }
}

fn extract_datetimes(text: &str) -> Vec<NaiveDateWithOptionalTime> {
  let mut results: Vec<NaiveDateWithOptionalTime> = Vec::new();
  
  const DAY_OF_MONTH: &str = r"0?[1-9]|[12][0-9]|3[01]";
  const NUMERIC_MONTH: &str = r"0?[1-9]|1[0-2]";
  trait DateExtractor {
    fn get_regexp(&self) -> regex::Regex;
    fn extract(&self, captures: regex::Captures) -> NaiveDateWithOptionalTime;
  }

  struct NumericExtractor {}
  impl DateExtractor for NumericExtractor {
    fn get_regexp(&self) -> regex::Regex {
      Regex::new(&format!(r"({})\.({})\.", DAY_OF_MONTH, NUMERIC_MONTH)).unwrap()
    }

    fn extract(&self, captures: regex::Captures) -> NaiveDateWithOptionalTime {
      let month = captures[2].parse::<u32>().unwrap();
      let day = captures[1].parse::<u32>().unwrap();
      return NaiveDate::from_ymd(2022, month, day).into();
    }
  }

  struct TextExtractor { }
  impl DateExtractor for TextExtractor {
    fn get_regexp(&self) -> regex::Regex {
      let german_month_to_num: HashMap<&str, i32> = HashMap::from([
        ("Januar", 1),
        ("Februar", 2),
        ("März", 3),
        ("April", 4),
        ("Mai", 5),
        ("Juni", 6),
        ("Juli", 7),
        ("August", 8),
        ("September", 9),
        ("Oktober", 10),
        ("November", 11),
        ("Dezember", 12),
      ]);
      let month_names: Vec<&str> = german_month_to_num.keys().map(|k| *k).collect();
      let month_names_joined: &str = &month_names.join("|");
      Regex::new(&format!(r"({})\. ({})", DAY_OF_MONTH, month_names_joined)).unwrap()
    }

    fn extract(&self, captures: regex::Captures) -> NaiveDateWithOptionalTime {
      let german_month_to_num: HashMap<&str, u32> = HashMap::from([
        ("Januar", 1),
        ("Februar", 2),
        ("März", 3),
        ("April", 4),
        ("Mai", 5),
        ("Juni", 6),
        ("Juli", 7),
        ("August", 8),
        ("September", 9),
        ("Oktober", 10),
        ("November", 11),
        ("Dezember", 12),
      ]);
      let month = german_month_to_num[&captures[2]];
      let day = captures[1].parse::<u32>().unwrap();
      return NaiveDate::from_ymd(2022, month, day).into();
    }
  }

  let regexps: Vec<&dyn DateExtractor> = vec![&NumericExtractor{}, &TextExtractor{}];
  // println!("{}\n", text);
  for regexp in regexps {
    for captured in regexp.get_regexp().captures_iter(&text) {
      results.push(regexp.extract(captured));
    }
    if results.len() > 0 {
      break;
    }
  }

  return results;
}

fn extract_text(div: &ElementRef) -> String {
  let all_text: String = div.text().collect::<String>();
  // Replace all sequences of whitespaces with a single white space
  Regex::new(r"\s+").unwrap().replace_all(&all_text, " ").to_string()
}
