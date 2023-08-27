use std::path::Path;
use gelbooru_api::{Client as GClient, posts};
use gelbooru_api::api::{Post, PostQuery};
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folder: String = std::env::args().nth(1).expect("Please provide a destination folder path.");
    if !Path::new(&folder).is_dir() {
        return Err("Please provide a valid path.".into());
    }

    let tags: Vec<String> = std::env::args().skip(2).collect();
    if tags.is_empty() {
        return Err(format!(
            "Usage: {} <dest-folder-path> <tags separated with space>",
            std::env::args().nth(0).unwrap(),
        ).into());
    }

    let client = GClient::public();

    let mut request = posts();
    for tag in tags { request = request.tag(tag); }

    println!("Fetching Information.. ⏳");
    let PostQuery { posts, .. } = request.send(&client).await?;
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots().https_only()
        .enable_http1().build();
    let http_client = hyper::Client::builder().build::<_, hyper::Body>(connector);

    let length = posts.len();
    for (idx, post) in posts.into_iter().enumerate() {
        tokio_scoped::scope(|scope| {
            print!("[{idx} of {length}] ");
            scope.spawn(async {
                deploy(
                    http_client.clone(),
                    post,
                    &folder
                ).await
                    .unwrap()
            });
        });
    }

    Ok(println!("Downloaded at folder `{folder}`. ✅"))
}

async fn deploy(client: Client<HttpsConnector<HttpConnector>>, post: Post, folder: impl AsRef<str>) -> Result<(), Box<dyn std::error::Error>> {
    let folder = folder.as_ref();

    let Post { file_url, image, .. } = post;
    println!("Downloading `{image}`.. ⏳");

    let file_url = file_url.parse::<hyper::Uri>()?;

    let contents = hyper::body::to_bytes(
        client.get(file_url).await?.into_body()
    ).await?.to_vec();

    tokio::fs::write(
        Path::new(&std::format!("{folder}/{image}")),
        contents
    ).await.map_err(Into::into)
}
