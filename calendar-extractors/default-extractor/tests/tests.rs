use std::{fs, path::PathBuf};

use default_extractor::extractors::{smallest_div_with_date::SmallestDivWithDateExtractor, EventExtractor};

#[test]
#[ignore]
fn test_no_events_from_google_landing_page() {
  let test_response = get_test_response("google.html");
  println!("{}", test_response);

  let results = SmallestDivWithDateExtractor::code_to_events(&test_response);
  assert!(results.is_empty());
}

#[test]
fn test_some_events_from_theater_essen() {
  let test_response = get_test_response("theater-essen.html");

  let results = SmallestDivWithDateExtractor::code_to_events(&test_response);
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

#[test]
fn test_some_events_from_ruhr_museum() {
  let test_response = get_test_response("ruhr-museum.html");

  let results = SmallestDivWithDateExtractor::code_to_events(&test_response);
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

#[test]
fn test_some_events_from_tonhalle() {
  let test_response = get_test_response("tonhalle.html");

  let results = SmallestDivWithDateExtractor::code_to_events(&test_response);
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

#[test]
// To get this test to work, you'll need to switch from HTTP request to headless browser executing the JS.
fn test_some_events_from_radio_essen() {
  let test_response = get_test_response("radio-essen.html");

  let results = SmallestDivWithDateExtractor::code_to_events(&test_response);
  assert!(10 < results.len());
  assert!(results.len() < 70);
}

fn get_test_response(name: &str) -> String {
  let mut test_response_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  test_response_file.push("tests/test_responses/");
  test_response_file.push(name);

  fs::read_to_string(test_response_file).unwrap()
}