use reqwest::{Proxy, Client};
use select::document::Document;
use select::predicate::{Any};
use std::env;

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

    // Create a reqwest client with SOCKS5 proxy pointing to TOR proxy port
    let proxy = Proxy::all("socks5h://127.0.0.1:9050").expect("Failed to create proxy");
    let client = Client::builder()
        .proxy(proxy)
        .build()
        .expect("Failed to build client");

    // Fetch the content from the URL
    let res = client
        .get(url)
        .send()
        .await
        .expect("Request failed")
        .text()
        .await
        .expect("Failed to read response text");

    // Parse the HTML document
    let document = Document::from(res.as_str());

    // Extract and print the contents of the specified tags
    for node in document.find(Any) {
        if let Some(tag) = node.name() {
            if tags.contains(&tag.as_ref()) {
                println!("<{}>: {}", tag, node.text());
            }
        }
    }
}
