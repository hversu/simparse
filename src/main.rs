use {http_req::error, http_req::request};
use std::env;
use scraper;

fn main() {
    let args: Vec<String> = env::args().collect();
    let site = &args[1];
    let tag = &args[2];
    println!("site:{}, tags:{}", &site, tag);
    let result = scrape(site, tag);
    println!("{:#?}", result.unwrap())
}

fn scrape(url: &str, tag: &str) -> Result<Vec<String>, error::Error> {
    let result = call(url);
    let parsed = parse(&result?, tag);
    return parsed
}

fn parse(html: &str, tag: &str) -> Result<Vec<String>, error::Error> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse(tag).unwrap();
    let content = document.select(&selector)
                          .map(|element| element.inner_html())
                          .collect::<Vec<_>>();
    return Ok(content)
}

fn call(url: &str) -> Result<String, error::Error> {
   let mut response_body = Vec::new();
   request::get(url, &mut response_body).map_err(|e| e)?;
   Ok(String::from_utf8_lossy(&response_body).to_string())
}
