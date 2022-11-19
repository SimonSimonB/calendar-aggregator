use std::{fs, path::PathBuf};

use chrono::NaiveDate;
use default_extractor::extractors::{
    largest_element_with_single_date::LargestElementWithSingleDateExtractor, EventExtractor,
};

#[test]
fn test_some_events_from_theater_essen() {
    let test_html = get_html("theater-essen.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
}

#[test]
fn test_some_events_from_ruhr_museum() {
    let test_html = get_html("ruhr-museum.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Geschichte"))
        .is_some())
}

#[test]
fn test_some_events_from_tonhalle() {
    let test_html = get_html("tonhalle.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Vivaldi"))
        .is_some())
}

#[test]
fn test_some_events_from_koelner_philharmonie() {
    let test_html = get_html("koelner-philharmonie.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Haydn"))
        .is_some())
}

#[test]
fn test_some_events_from_elbphilharmonie() {
    let test_html = get_html("elbphilharmonie.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("The Philadelphia Orchestra"))
        .is_some())
}

#[test]
fn test_some_events_from_gruga() {
    let test_html = get_html("gruga.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Farbenpracht der Dahlie"))
        .is_some())
}

#[test]
#[ignore]
// To get this test to work, you'll need to switch from HTTP request to headless browser executing the JS.
fn test_some_events_from_theater_hamburg() {
    let test_html = get_html("theater-hamburg.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Footloose"))
        .is_some())
}

#[test]
#[ignore]
// To get this test to work, you'll need to switch from HTTP request to headless browser executing the JS.
fn test_some_events_from_radio_essen() {
    let test_html = get_html("radio-essen.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 8, 1),
    );
    assert!(10 < results.len());
    assert!(results.len() < 70);
}

#[test]
fn test_fake_website() {
    let test_html = get_html("fake-website.html");

    let results = LargestElementWithSingleDateExtractor::html_to_events(
        &test_html,
        &NaiveDate::from_ymd(2022, 1, 1),
    );
    assert!(results.len() == 2);
    assert!(results
        .into_iter()
        .find(|r| r.text.contains("Taylor Swift"))
        .is_some())
}

fn get_html(name: &str) -> String {
    let mut html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    html_path.push("tests/test_html/");
    html_path.push(name);

    fs::read_to_string(html_path).unwrap()
}
