use reqwest;
use reqwest::header;
use scraper::{Html, Selector};
use std::io;
use urlencoding::encode;

/**
* This program gets input from the user, turns it into a url-safe string, and then searches Walmart.com for that string.
* Then it prints the names and prices of all products that match the search.
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get input from the user on what to query
    let mut input = String::new();
    println!("Can you enter something to query Walmart.com for (Ex: laptop)?");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // URL encode the input
    let query = encode(input.trim());

    println!("URL-safe string: {}", query);

    // Define the URL you want to scrape
    let full_url = format!("{}{}", "https://www.walmart.com/search/?query=", query);

    println!("Requesting Walmart.com...");

    // Request and pretend we're on google to avoid robot detection
    let client = reqwest::Client::new();
    let response = client
        .get(full_url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .header(header::REFERER, "https://www.google.com/")
        .send()
        .await?;
    if !response.status().is_success() {
        eprintln!("Request was not successful. Status code: {:?}", response.status());
        return Ok(());
    } else {
        println!("Got a response from Walmart.com!");
    }

    // Parse the HTML content from the response
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    // Selectors for name, price, and products
    let product_name_selector = Selector::parse("[data-automation-id=product-title]").unwrap();
    let product_price_selector_current_and_was = Selector::parse("[data-automation-id=product-price]").unwrap();
    let product_selector = Selector::parse("[data-item-id]").unwrap();

    println!("Printing the names and prices of all products...");

    // Extract and print the product names and prices
    for product in document.select(&product_selector) {
        let name = product.select(&product_name_selector).next().unwrap().text().collect::<String>();
        let prices: Vec<String> = document
            .select(&product_price_selector_current_and_was)
            .map(|element| element.text().collect())
            .collect();
        let prices_string = prices
            .iter()
            .map(|price| format!("Price: {}", price))
            .collect::<Vec<_>>()
            .join("\n");

        println!("Product: {}\nPrice: {}\n", name, prices_string);
    }

    Ok(())
}