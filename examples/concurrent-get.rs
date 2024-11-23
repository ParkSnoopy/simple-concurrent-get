#[tokio::main]
async fn main() {
    let request_urls = vec![
        "https://example.com/1",
        "https://example.com/2",
        "https://example.com/3",
        "https://example.com/4",
        // ...
    ];
    // maximum 3 concurrent request
    let request_results = simple_concurrent_get::concurrent_get(request_urls, 3);
    let _: Vec<_> = request_results
        .await
        .into_iter()
        .map(|result| match result {
            Ok(response) => {
                let url = response.url().to_owned();
                let bytes = futures_executor::block_on(async{ response.bytes().await }).unwrap();
                println!("Successfully got '{}' with '{}' bytes of content", url, bytes.len())
            },
            Err(e) => eprintln!("{}", e),
        })
        .collect();
}
