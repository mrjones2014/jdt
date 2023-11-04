use image_repo::download_repo_manifest;
use std::env;

#[tokio::main]
pub async fn main() {
    let urls = env::args().skip(1);

    if urls.len() == 0 {
        eprintln!("Usage: cargo run -p image-repo --example cli -- [url]");
    }

    for url in urls {
        println!("Attempting to download repo from {url} ...");
        let result = download_repo_manifest(url).await;
        if let Ok(repo) = result {
            println!("{}", serde_json::to_string_pretty(&repo).unwrap());
        } else {
            println!("{result:?}");
        }
    }
}
