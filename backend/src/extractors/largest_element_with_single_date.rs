use chrono::NaiveDate;
use regex::Regex;
use scraper::ElementRef;

use super::date_extraction;
use super::Event;
use super::EventExtractor;

pub struct LargestElementWithSingleDateExtractor {}

impl EventExtractor for LargestElementWithSingleDateExtractor {
    fn code_to_events(website_code: &str, from_date: &NaiveDate) -> Vec<Event> {
        let document = scraper::Html::parse_document(website_code);
        let start_elements_to_try = ["main", "body", "html"];
        for start_element in start_elements_to_try {
            let main_selector = scraper::Selector::parse(start_element).unwrap();
            let start_elements: Vec<ElementRef> =
                document.select(&main_selector).collect::<Vec<ElementRef>>();
            if start_elements.len() > 0 {
                let mut events = element_to_events(&start_elements[0]);
                events = events
                    .into_iter()
                    .filter(|event| event.date.lt(from_date))
                    .collect();
                return events;
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

    // If no events were found in any of the children, then try to extract an event from the totality of
    // text in this element.
    if events.is_empty() {
        if let Some(event) = element_to_one_event(el) {
            events = vec![event];
        }
    }
    // If one event was extracted from all of the children, and just one date is found below this node
    // then try to extract this event again, but from this node, i.e., with more text.
    else if events.len() == 1 {
        let dates = date_extraction::extract_datetimes(el);
        if dates.len() == 1 {
            if let Some(event) = element_to_one_event(el) {
                events = vec![event];
            }
        }
    }

    events
}

fn element_to_one_event(el: &ElementRef) -> Option<Event> {
    let dates = date_extraction::extract_datetimes(el);

    if dates.len() != 1 {
        return None;
    }
    let date = &dates[0];

    let text = extract_text(el);
    if text.len() <= 15 {
        return None;
    }

    Some(Event::new(&text, date.date))
}

fn extract_text(div: &ElementRef) -> String {
    let all_text: String = div.text().map(|s| format!("{} ", s)).collect::<String>();
    // Replace all sequences of whitespaces with a single white space
    return Regex::new(r"\s+")
        .unwrap()
        .replace_all(&all_text, " ")
        .to_string();
}
