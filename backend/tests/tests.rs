use std::{fs, path::PathBuf};

use chrono::NaiveDate;
use default_extractor::extractors::{largest_element_with_single_date::LargestElementWithSingleDateExtractor, EventExtractor};

#[test]
fn test_some_events_from_theater_essen() {
  let test_response = get_test_response("theater-essen.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

#[test]
fn test_some_events_from_ruhr_museum() {
  let test_response = get_test_response("ruhr-museum.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("Geschichte")).is_some())
}

#[test]
fn test_some_events_from_tonhalle() {
  let test_response = get_test_response("tonhalle.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("Vivaldi")).is_some())
}

#[test]
fn test_some_events_from_koelner_philharmonie() {
  let test_response = get_test_response("koelner-philharmonie.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("Haydn")).is_some())
}

#[test]
fn test_some_events_from_elbphilharmonie() {
  let test_response = get_test_response("elbphilharmonie.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("The Philadelphia Orchestra")).is_some())
}

#[test]
fn test_some_events_from_gruga() {
  let test_response = get_test_response("gruga.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("Farbenpracht der Dahlie")).is_some())
}

#[test]
#[ignore]
// To get this test to work, you'll need to switch from HTTP request to headless browser executing the JS.
fn test_some_events_from_theater_hamburg() {
  let test_response = get_test_response("theater-hamburg.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
  assert!(results.into_iter().find(|r| r.text.contains("Footloose")).is_some())
}

#[test]
#[ignore]
// To get this test to work, you'll need to switch from HTTP request to headless browser executing the JS.
fn test_some_events_from_radio_essen() {
  let test_response = get_test_response("radio-essen.html");

  let results = LargestElementWithSingleDateExtractor::code_to_events(&test_response, &NaiveDate::from_ymd(2022, 8, 1));
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

fn get_test_response(name: &str) -> String {
  let mut test_response_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  test_response_file.push("tests/test_responses/");
  test_response_file.push(name);

  fs::read_to_string(test_response_file).unwrap()
}