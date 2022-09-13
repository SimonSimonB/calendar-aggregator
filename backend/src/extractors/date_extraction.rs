use std::collections::HashMap;

use chrono::NaiveDate;
use regex::Regex;
use scraper::ElementRef;

use super::NaiveDateWithOptionalTime;

pub struct DateMatch {
  pub date: NaiveDateWithOptionalTime,
  pub matched_string: String,
}

const DAY_OF_MONTH: &str = r"0?[1-9]|[12][0-9]|3[01]";
const NUMERIC_MONTH: &str = r"0?[1-9]|1[0-2]";
trait DateExtractor {
  fn get_regexp(&self) -> regex::Regex;
  fn extract(&self, captures: regex::Captures) -> Option<NaiveDateWithOptionalTime>;
}

struct DateWithNumericMonthExtractor {}
impl DateExtractor for DateWithNumericMonthExtractor {
  fn get_regexp(&self) -> regex::Regex {
    Regex::new(&format!(r"({})\.({})\.", DAY_OF_MONTH, NUMERIC_MONTH)).unwrap()
  }

  fn extract(&self, captures: regex::Captures) -> Option<NaiveDateWithOptionalTime> {
    let month = captures[2].parse::<u32>().unwrap();
    let day = captures[1].parse::<u32>().unwrap();

    match NaiveDate::from_ymd_opt(2022, month, day) {
      None => None,
      Some(e) => Some(e.into()),
    }
  }
}

struct DateWithGermanMonthExtractor { }
impl DateWithGermanMonthExtractor {
  fn get_month_to_num() -> HashMap<&'static str, u32> {
    let mut german_month_to_num: HashMap<&str, u32> = HashMap::from([
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
    let mut to_add = Vec::new();
    for (k, v) in german_month_to_num.clone() {
      to_add.push((k, v));
    }
    for (k, v) in to_add {
      let s: &str = &k[..3];
      german_month_to_num.insert(s, v);
    }
    return german_month_to_num;
  }
}

impl DateExtractor for DateWithGermanMonthExtractor {
  fn get_regexp(&self) -> regex::Regex {
    let german_month_to_num = DateWithGermanMonthExtractor::get_month_to_num();
    let month_names: Vec<&str> = german_month_to_num.keys().map(|k| *k).collect();
    let month_names_joined: &str = &month_names.join("|");
    Regex::new(&format!(r"({})\.?\s?({})", DAY_OF_MONTH, month_names_joined)).unwrap()
  }

  fn extract(&self, captures: regex::Captures) -> Option<NaiveDateWithOptionalTime> {
    let german_month_to_num = DateWithGermanMonthExtractor::get_month_to_num();
    let month = german_month_to_num[&captures[2]];
    let day = captures[1].parse::<u32>().unwrap();
    println!("Month: {}\nDay: {}", month, day);
    
    match NaiveDate::from_ymd_opt(2022, month, day) {
      None => None,
      Some(e) => Some(e.into()),
    }
  }
}

pub fn extract_datetimes(div: &ElementRef) -> Vec<DateMatch> {
  let mut text: String = div.text().collect();
  text = Regex::new(r"\s+").unwrap().replace_all(&text, " ").to_string();
  let mut results: Vec<DateMatch> = Vec::new();

  let date_extractors: Vec<&dyn DateExtractor> = vec![&DateWithNumericMonthExtractor{}, &DateWithGermanMonthExtractor{}];
  println!("{}\n", text);
  for date_extractor in date_extractors {
    for captured in date_extractor.get_regexp().find_iter(&text).zip(date_extractor.get_regexp().captures_iter(&text)) {
      match date_extractor.extract(captured.1) {
        None => (),
        Some(date) => results.push(DateMatch { date: date, matched_string: captured.0.as_str().to_owned() })
      }
    }
    if results.len() > 0 {
      break;
    }
  }

  return results;
}