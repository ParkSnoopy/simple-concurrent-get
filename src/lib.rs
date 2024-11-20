use std::sync::{ Arc, RwLock };
use futures_util::stream::{ StreamExt as _ };



async fn get<S: AsRef<str>>(url: S) -> ehttp::Result<ehttp::Response> {
    let request = ehttp::Request::get(url.as_ref());
    ehttp::fetch_async(request).await
}

pub async fn concurrent_get<I,S>(fetch_urls: I, concurrent: usize) -> Vec<ehttp::Result<ehttp::Response>>
where
    S: AsRef<str>,
    I: IntoIterator<Item=S>,
{
    // Initialize Response Container ( with initial capacity of 10x concurrent )
    let results = Arc::new(RwLock::new(Vec::new()));

    let bodies = futures_util::stream::iter(fetch_urls)
        .map(|url| {
            async move {
                get(url).await
            }
        })
        .buffer_unordered(concurrent);

    bodies
        .for_each(|resp| {
            let results_cloned = results.clone();
            async move {
                results_cloned.write().unwrap().push(resp);
            }
        })
        .await;

    let results = Arc::try_unwrap(results).unwrap();
    let results = results.into_inner().unwrap();

    results
}

pub async fn concurrent_get_foreach<I,S,F,R>(fetch_urls: I, concurrent: usize, run_for_each: F) -> Vec<Result<R, String>>
where
    S: AsRef<str>,
    I: IntoIterator<Item=S>,
    F: Copy + FnOnce(ehttp::Response) -> R,
    R: std::fmt::Debug
{
    // Initialize Response Container ( with initial capacity of 10x concurrent )
    let results = Arc::new(RwLock::new(Vec::new()));

    let bodies = futures_util::stream::iter(fetch_urls)
        .map(|url| {
            async move {
                get(url).await
            }
        })
        .buffer_unordered(concurrent);

    bodies
        .for_each(|resp| {
            let results = results.clone();
            async move {
                match resp {
                    Ok(resp) => results
                        .write().unwrap()
                        .push( Ok(run_for_each(resp)) ),
                    Err(resp) => results
                        .write().unwrap()
                        .push( Err(resp) )
                }
            }
        })
        .await;

    let results = Arc::try_unwrap(results).unwrap();
    let results = results.into_inner().unwrap();

    results
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
                (parse_text_into_length(resp.text().unwrap()), resp.url)
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
                |resp| {
                    (parse_text_into_length(resp.text().unwrap()), resp.url)
                }
            )
            .await
            .into_iter()
            .map(|r| r.unwrap())
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
