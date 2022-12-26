use futures_lite::future;

fn main() {
    let reqs = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org".to_string(),
    ];

    let results = future::block_on(async_get::get(reqs));
    for result in results {
        match result {
            Ok(resp) => println!("*** {}\n", resp),
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
