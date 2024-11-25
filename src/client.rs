use reqwest::Client;

use crate::config;



pub fn build_preset() -> Client {
    Client::builder()
        .user_agent(config::USER_AGENT)
        .gzip(true)
        .deflate(true)
        .build()
        .unwrap()
}
