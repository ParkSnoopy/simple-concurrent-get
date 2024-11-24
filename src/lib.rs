//! # simple-concurrent-get
//!
//! Make multiple concurrent HTTP GET requests with ease.
//!
//! ## Making a GET request
//!
//! using `concurrent_get`
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     let request_urls = vec![
//!         "https://example.com/1",
//!         "https://example.com/2",
//!         "https://example.com/3",
//!         "https://example.com/4",
//!         // ...
//!     ];
//!
//!     // for example: maximum 3 concurrent request
//!     let _: Vec<_> = simple_concurrent_get::concurrent_get(request_urls, 3)
//!         .await
//!         .map(|result| match result {
//!             Ok(response) => {
//!                 let url = response.url().to_owned();
//!                 let bytes = futures_executor::block_on(async{ response.bytes().await }).unwrap();
//!                 println!("Successfully got '{}' with '{}' bytes of content", url, bytes.len())
//!             },
//!             Err(e) => eprintln!("{}", e),
//!         })
//!         .collect();
//! }
//! ```
//!
//! using `concurrent_get_foreach`
//! ( which is simply passing process function as argument )
//! ``` rust
//! #[tokio::main]
//! async fn main() {
//!     let request_urls = vec![
//!         "https://example.com/1",
//!         "https://example.com/2",
//!         "https://example.com/3",
//!         "https://example.com/4",
//!         // ...
//!     ];
//!
//!     // for example: maximum 3 concurrent request
//!     // ensure response is in order
//!     let _: Vec<_> = simple_concurrent_get::ordered::concurrent_get_foreach(request_urls, 3,
//!         |result| match result {
//!             Ok(response) => {
//!                 let url = response.url().to_owned();
//!                 let bytes = futures_executor::block_on(async{ response.bytes().await }).unwrap();
//!                 println!("Successfully got '{}' with '{}' bytes of content", url, bytes.len())
//!             },
//!             Err(e) => eprintln!("{}", e),
//!         }
//!     )
//!        .await
//!        .collect(); 
//!}
//!```
//!



mod config;

pub mod unordered;
pub mod ordered;

#[cfg(test)]
mod tests;

// unordered as default
pub use unordered::*;
