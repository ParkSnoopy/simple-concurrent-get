use reqwest::{ Client, Response };
use futures_util::stream::{ StreamExt as _ };

use std::sync::Arc;
use std::sync::mpsc::{
    IntoIter,
    channel,
};

use crate::config;



pub async fn concurrent_get_foreach<I,S,F,R>(fetch_urls: I, concurrent: usize, run_for_each: F) -> IntoIter<R>
where
    S: reqwest::IntoUrl,
    I: IntoIterator<Item=S>,
    F: Copy + FnOnce(reqwest::Result<Response>) -> R,
{
    let client: Arc<Client> = Arc::new(Client::builder()
        .user_agent(config::USER_AGENT)
        .build()
        .unwrap());

    let (sender, receiver) = channel();

    let bodies = futures_util::stream::iter(fetch_urls)
        .map(|url| {
            let client = client.clone();
            async move {
                run_for_each(
                    client.get(url).send().await
                )
            }
        })
        .buffer_unordered(concurrent);

    bodies
        .for_each(|resp| {
            let sender = sender.clone();
            async move {
                sender.send(resp).unwrap();
            }
        })
        .await;

    receiver.into_iter()
}
