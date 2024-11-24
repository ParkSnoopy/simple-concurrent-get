use reqwest::{ Client, Response };
use futures_util::stream::{ StreamExt as _ };

use std::sync::Arc;
use std::sync::mpsc::{
    IntoIter,
    channel,
};

use crate::{
    config,
};



pub async fn concurrent_get<I,S>(fetch_urls: I, concurrent: usize) -> IntoIter<reqwest::Result<Response>>
where
    I: IntoIterator<Item=S>,
    S: reqwest::IntoUrl,
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
                client.get(url).send().await
            }
        })
        .buffered(concurrent);

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
