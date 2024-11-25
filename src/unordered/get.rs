use reqwest::{ Client, Response };
use futures_util::stream::{ StreamExt as _ };

use std::sync::Arc;
use std::sync::mpsc::{
    IntoIter,
    channel,
};

use crate::{
    client,
};



pub async fn concurrent_get<I,S>(fetch_urls: I, concurrent: usize) -> IntoIter<reqwest::Result<Response>>
where
    I: IntoIterator<Item=S>,
    S: reqwest::IntoUrl,
{
    let client: Arc<Client> = Arc::new(client::build_preset());

    let (sender, receiver) = channel();

    let results = futures_util::stream::iter(fetch_urls)
        .map(|url| {
            let client = client.clone();
            async move {
                client.get(url).send().await
            }
        })
        .buffer_unordered(concurrent);

    results
        .for_each(|resp| {
            let sender = sender.clone();
            async move {
                sender.send(resp).unwrap();
            }
        })
        .await;

    drop(sender);
    receiver.into_iter()
}
