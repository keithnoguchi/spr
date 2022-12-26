use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn get(urls: &[String]) -> Vec<Result<String>> {
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls {
        let req = client.get(url).recv_string();
        handles.push(async_std::task::spawn(req));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.map_err(|e| e.into()));
    }
    results
}
