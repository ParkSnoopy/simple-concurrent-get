use super::utils::{
    make_url_lownull,
    parse_text_into_length,
};

use crate::ordered::{
    concurrent_get,
    concurrent_get_foreach,
};

const TO_FETCH: &[&str; 11] = &[
    "Amarr:Jita:Dodixie:Hek:Rens:Nisuwa",
    "Jita:Dodixie:Hek:Rens:Nisuwa",
    "Amarr:Dodixie:Hek:Rens:Nisuwa",
    "Amarr:Jita:Hek:Rens:Nisuwa",
    "Amarr:Jita:Dodixie:Rens:Nisuwa",

    "Covryn:Brarel:Frarie:Renarelle:Vivanier",
    "Covryn:Brarel:Frarie:Renarelle:Nisuwa",
    "Covryn:Brarel:Frarie:Vivanier:Nisuwa",
    "Covryn:Brarel:Renarelle:Vivanier:Nisuwa",
    "Covryn:Frarie:Renarelle:Vivanier:Nisuwa",
    "Brarel:Frarie:Renarelle:Vivanier:Nisuwa",
];
const CONCURRENT: usize = 2;



#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_ordered_concurrent_get() {
    let fetch_urls_iter = TO_FETCH
        .iter()
        .map(make_url_lownull);

    let results: Vec<(u64, String)> = concurrent_get(fetch_urls_iter, CONCURRENT)
        .await
        .map(|result| {
            let resp = result.unwrap();
            let url: String = resp.url().to_string();
            let body_test = futures_executor::block_on(async{ resp.text().await }).unwrap();
            (
                parse_text_into_length(body_test),
                url,
            )
        })
        .collect();

    assert_eq!(
        results,
        vec![
            (106, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens:Nisuwa"      .to_string() ),
            (66 , "https://evemaps.dotlan.net/route/3:Jita:Dodixie:Hek:Rens:Nisuwa"            .to_string() ),
            (86 , "https://evemaps.dotlan.net/route/3:Amarr:Dodixie:Hek:Rens:Nisuwa"           .to_string() ),
            (79 , "https://evemaps.dotlan.net/route/3:Amarr:Jita:Hek:Rens:Nisuwa"              .to_string() ),
            (98 , "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Rens:Nisuwa"          .to_string() ),

            (20 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Renarelle:Vivanier" .to_string() ),
            (19 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Renarelle:Nisuwa"   .to_string() ),
            (29 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Vivanier:Nisuwa"    .to_string() ),
            (33 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Renarelle:Vivanier:Nisuwa" .to_string() ),
            (29 , "https://evemaps.dotlan.net/route/3:Covryn:Frarie:Renarelle:Vivanier:Nisuwa" .to_string() ),
            (27 , "https://evemaps.dotlan.net/route/3:Brarel:Frarie:Renarelle:Vivanier:Nisuwa" .to_string() ),
        ]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_ordered_concurrent_get_foreach() {
    let fetch_urls_iter = TO_FETCH
        .iter()
        .map(make_url_lownull);

    let results: Vec<(u64, String)> = concurrent_get_foreach(
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
        .collect();

    assert_eq!(
        results,
        vec![
            (106, "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Hek:Rens:Nisuwa"      .to_string() ),
            (66 , "https://evemaps.dotlan.net/route/3:Jita:Dodixie:Hek:Rens:Nisuwa"            .to_string() ),
            (86 , "https://evemaps.dotlan.net/route/3:Amarr:Dodixie:Hek:Rens:Nisuwa"           .to_string() ),
            (79 , "https://evemaps.dotlan.net/route/3:Amarr:Jita:Hek:Rens:Nisuwa"              .to_string() ),
            (98 , "https://evemaps.dotlan.net/route/3:Amarr:Jita:Dodixie:Rens:Nisuwa"          .to_string() ),

            (20 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Renarelle:Vivanier" .to_string() ),
            (19 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Renarelle:Nisuwa"   .to_string() ),
            (29 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Frarie:Vivanier:Nisuwa"    .to_string() ),
            (33 , "https://evemaps.dotlan.net/route/3:Covryn:Brarel:Renarelle:Vivanier:Nisuwa" .to_string() ),
            (29 , "https://evemaps.dotlan.net/route/3:Covryn:Frarie:Renarelle:Vivanier:Nisuwa" .to_string() ),
            (27 , "https://evemaps.dotlan.net/route/3:Brarel:Frarie:Renarelle:Vivanier:Nisuwa" .to_string() ),
        ]
    );
}
