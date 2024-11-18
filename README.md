# simple-concurrent-get

[![Latest version](https://img.shields.io/crates/v/simple-concurrent-get.svg)](https://crates.io/crates/simple-concurrent-get)
[![Documentation](https://docs.rs/simple-concurrent-get/badge.svg)](https://docs.rs/simple-concurrent-get)
![MIT](https://img.shields.io/badge/license-GPLv3-blue.svg)

Make multiple concurrent HTTP GET requests with ease.
Check [ehttp::Response](https://docs.rs/ehttp/latest/ehttp/struct.Response.html) for response type.

## Usage
``` rust
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
            Ok(response) => println!("{}", response.url),
            Err(e) => eprintln!("{:?}", e),
        })
        .collect();
}
```
