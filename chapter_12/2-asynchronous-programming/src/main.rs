use reqwest::Client;

async fn fetch_data() -> Result<String, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts";

    let client = Client::new();
    let response = client.get(url).send().await?;

    let body = response.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Fetched data: {}", data),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
