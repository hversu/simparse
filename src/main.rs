use reqwest::{Proxy, Client};
use select::document::Document;
use select::predicate::Any;
use std::env;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct TagValuePair {
    tag: String,
    value: String,
}

pub async fn fetch_and_extract(url: &str, tags: Vec<&str>) -> Result<Vec<TagValuePair>, Box<dyn Error>> {
    // Create a reqwest client with SOCKS5 proxy pointing to TOR proxy port
    let proxy = Proxy::all("socks5h://127.0.0.1:9050")?;
    let client = Client::builder()
        .proxy(proxy)
        .build()?;

    // Fetch the content from the URL
    let res = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    // Parse the HTML document
    let document = Document::from(res.as_str());

    // Extract the contents of the specified tags or paths
    let mut results = Vec::new();
    for node in document.find(Any) {
        if let Some(tag) = node.name() {
            for &tag_spec in &tags {
                if tag_spec.contains('.') {
                    let parts: Vec<&str> = tag_spec.split('.').collect();
                    if parts.len() == 2 && tag == parts[0] {
                        if let Some(attr_value) = node.attr(parts[1]) {
                            results.push(TagValuePair {
                                tag: tag_spec.to_string(),
                                value: attr_value.to_string(),
                            });
                        }
                    }
                } else if tag == tag_spec {
                    results.push(TagValuePair {
                        tag: tag.to_string(),
                        value: node.text(),
                    });
                }
            }
        }
    }

    Ok(results)
}

#[tokio::main]
async fn main() {
    // Retrieve the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run <url> <tags_comma_separated>");
        return;
    }

    let url = &args[1];
    let tags: Vec<&str> = args[2].split(',').collect();

    match fetch_and_extract(url, tags).await {
        Ok(results) => {
            let json_results: Vec<_> = results.iter()
                .map(|result| json!({ "tag": &result.tag, "value": &result.value }))
                .collect();
            
            println!("{}", serde_json::to_string_pretty(&json_results).unwrap());
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
