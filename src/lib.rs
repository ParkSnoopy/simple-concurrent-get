use reqwest::{ Client, Response };
use futures_util::stream::{ StreamExt as _ };

use std::sync::{ Arc, RwLock };

const USER_AGENT: &str = "simple-concurrent-get/v0.2";



pub async fn concurrent_get<I,S>(fetch_urls: I, concurrent: usize) -> Vec<reqwest::Result<Response>>
where
    S: reqwest::IntoUrl,
    I: IntoIterator<Item=S>,
{
    let client: Arc<Client> = Arc::new(Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap());

    let results = Arc::new(RwLock::new(Vec::new()));

    let bodies = futures_util::stream::iter(fetch_urls)
        .map(|url| {
            let client = client.clone();
            async move {
                client.get(url).send().await
            }
        })
        .buffer_unordered(concurrent);

    bodies
        .for_each(|resp| {
            let results = results.clone();
            async move {
                results.write().unwrap().push(resp);
            }
        })
        .await;

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

pub async fn concurrent_get_foreach<I,S,F,R>(fetch_urls: I, concurrent: usize, run_for_each: F) -> Vec<R>
where
    S: reqwest::IntoUrl,
    I: IntoIterator<Item=S>,
    F: Copy + FnOnce(reqwest::Result<Response>) -> R,
    R: std::fmt::Debug
{
    let client: Arc<Client> = Arc::new(Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap());

    let results = Arc::new(RwLock::new(Vec::new()));

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
            let results = results.clone();
            async move {
                results.write().unwrap().push(resp);
            }
        })
        .await;

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}



#[cfg(test)]
mod tests {
    const TO_FETCH: &[&str; 7] = &[
        "Amarr:Jita:Dodixie:Hek:Rens:Nisuwa",
        "Jita:Dodixie:Hek:Rens:Nisuwa",
        "Amarr:Dodixie:Hek:Rens:Nisuwa",
        "Amarr:Jita:Hek:Rens:Nisuwa",
        "Amarr:Jita:Dodixie:Rens:Nisuwa",
        "Amarr:Jita:Dodixie:Hek:Nisuwa",
        "Amarr:Jita:Dodixie:Hek:Rens",
    ];
    const CONCURRENT: usize = 4;

    use scraper::{ Html, Selector };

    #[tokio::test]
    async fn test_concurrent_get() {
        use itertools::Itertools;

        let fetch_urls_iter = TO_FETCH
            .iter()
            .map(make_url_lownull);

        let results: Vec<(u64, String)> = crate::concurrent_get(fetch_urls_iter, CONCURRENT)
            .await
            .into_iter()
            .map(|result| {
                let resp = result.unwrap();
                let url: String = resp.url().to_string();
                let body_test = futures_executor::block_on(async{ resp.text().await }).unwrap();
                (
                    parse_text_into_length(body_test),
                    url,
                )
            })
            .sorted()
            .collect();

        assert_eq!(results,
            vec![
                (  66, "https://evemaps.dotlan.net/route/3:Jita:Dodixie:Hek:Rens:Nisuwa"     .to_string() ),
                (  79, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Hek:Rens:Nisuwa"       .to_string() ),
                (  86, "https://evemaps.dotlan.net/route/3:Amarr:Dodixie:Hek:Rens:Nisuwa"    .to_string() ),
                (  87, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens"      .to_string() ),
                (  92, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Nisuwa"    .to_string() ),
                (  98, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Rens:Nisuwa"   .to_string() ),
                ( 106,"https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens:Nisuwa".to_string() ),
            ]
        );
    }

    #[tokio::test]
    async fn test_concurrent_get_foreach() {
        use itertools::Itertools;

        let fetch_urls_iter = TO_FETCH
            .iter()
            .map(make_url_lownull);

        let results: Vec<(u64, String)> = crate::concurrent_get_foreach(
                fetch_urls_iter,
                CONCURRENT,
                |result| {
                    let resp = result.unwrap();
                    let url: String = resp.url().to_string();
                    let body_test = futures_executor::block_on(async{ resp.text().await }).unwrap();
                    (
                        parse_text_into_length(body_test),
                        url,
                    )
                }
            )
            .await
            .into_iter()
            .sorted()
            .collect();

        assert_eq!(results,
            vec![
                (  66, "https://evemaps.dotlan.net/route/3:Jita:Dodixie:Hek:Rens:Nisuwa"     .to_string() ),
                (  79, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Hek:Rens:Nisuwa"       .to_string() ),
                (  86, "https://evemaps.dotlan.net/route/3:Amarr:Dodixie:Hek:Rens:Nisuwa"    .to_string() ),
                (  87, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens"      .to_string() ),
                (  92, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Nisuwa"    .to_string() ),
                (  98, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Rens:Nisuwa"   .to_string() ),
                ( 106,"https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens:Nisuwa".to_string() ),
            ]
        );
    }

    fn make_url_lownull<S: AsRef<str>>(route: S) -> String {
        format!("https://evemaps.dotlan.net/route/3:{}",
            route.as_ref(),
        )
    }

    fn parse_text_into_length<S: AsRef<str>>(text: S) -> u64 {
        let distance: u64 = Html::parse_document(text.as_ref())
            .select(&Selector::parse(r#"div[id="navtools"]"#).unwrap())
            .next()
            .expect("Unexpected response format")
            .select(&Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap())
            .next()
            .expect("System Name Invalid")
            .select(&Selector::parse(r#"tr"#).unwrap())
            .last()
            .unwrap()
            .select(&Selector::parse(r#"td"#).unwrap())
            .next()
            .unwrap()
            .inner_html()
            .replace('.', "")
            .trim()
            .parse()
            .expect("Failed to parse route length");

        distance - 1
    }
}
