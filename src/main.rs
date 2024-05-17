use reqwest::Error;
use serde::Deserialize;
use std::io;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct NewsResponse {
    status: String,
    totalResults: Option<u32>,
    articles: Vec<Article>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    description: Option<String>,
    url: String,
    source: Source,
    publishedAt: String,
}

#[derive(Deserialize, Debug)]
struct Source {
    name: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get the user input
    println!("Enter a search term:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read line");
    let query = query.trim();

    // Get the user input for API key
    println!("Enter your NewsAPI API key:");
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read line");
    let api_key = api_key.trim();

    // Create the URL
    let url = format!("https://newsapi.org/v2/everything?q={}&apiKey={}", query, api_key);

    let client = reqwest::Client::new();
    // Send the request
    let response = client
        .get(&url)
        .header("User-Agent", "news-api-client")
        .send()
        .await?;

    // Attempt to parse the response as JSON
    let text = response.text().await?;
    match serde_json::from_str::<NewsResponse>(&text) {
        Ok(parsed_response) => {
            // Print the results
            println!("Status: {}", parsed_response.status);
            if let Some(total_results) = parsed_response.totalResults {
                println!("Total Results: {}", total_results);
            } else {
                println!("Total Results: Not provided");
            }
            for article in parsed_response.articles {
                println!("Title: {}", article.title);
                println!("{} | {}", article.source.name, article.publishedAt);
                if let Some(description) = article.description {
                    println!("Description: {}", description);
                } else {
                    println!("Description: Not provided");
                }
                println!("URL: {}\n", article.url);
            }
        }
        Err(e) => {
            // Print the raw response and the error
            println!("Failed to parse JSON response: {}\nResponse text: {}", e, text);
        }
    }

    Ok(())
}
