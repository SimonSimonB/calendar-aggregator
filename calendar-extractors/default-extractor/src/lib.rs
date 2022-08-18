use std::error::Error;

use extractors::{Event, smallest_div_with_date::SmallestDivWithDateExtractor, EventExtractor};
use reqwest::Url;

pub mod extractors;

async fn get(url: Url) -> Result<String, reqwest::Error> {
  reqwest::get(url).await?.text().await
}

pub async fn extract(url: &str) -> Result<Vec<Event>, Box<dyn Error>> {
  let url_parsed = Url::parse(url)?;
  let website_code = get(url_parsed).await?;
  Ok(SmallestDivWithDateExtractor::code_to_events(&website_code))
}