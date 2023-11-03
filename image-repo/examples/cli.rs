use image_repo::download_repo_manifest;
use std::env;

#[tokio::main]
pub async fn main() {
    for url in env::args().skip(1) {
        println!("Attempting to download repo from {url} ...");
        let result = download_repo_manifest(url).await;
        println!("Result:\n\n{result:?\n\n}");
    }
}
