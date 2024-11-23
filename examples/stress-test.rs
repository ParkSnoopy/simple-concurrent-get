use std::sync::{
    mpsc::channel,
};

const CONCURRENT: usize = 1000;
// successfully tested '33000' requests



#[tokio::main]
async fn main() {
    let (tx, rx) = channel();


    let request_urls = (1..)
        .take( CONCURRENT*CONCURRENT )
        .map(|n| format!("https://example.com/test/{}", n));

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .worker_threads(2)
            .thread_name("stress-test: task runner thread")
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let _: Vec<_> = simple_concurrent_get::concurrent_get_foreach(request_urls, CONCURRENT,
                |result| match result {
                    Ok(_r) => {
                        tx.send(true).unwrap();
                    },
                    Err(_e) => {
                        tx.send(false).unwrap();
                    },
                }
            )
                .await
                .into_iter()
                .collect();
        });
    });

    let mut fetch_counter: u128 = 0;
    for is_ok in rx.iter() {
        match is_ok {
            true => fetch_counter += 1,
            false => eprintln!("Fetch failed after '{fetch_counter}' successful fetches"),
        };
    }
}
