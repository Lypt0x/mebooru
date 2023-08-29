use std::path::{Path, PathBuf};
use gelbooru_api::{Client as GClient, posts};
use gelbooru_api::api::{Post, PostQuery};
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use clap::Parser;
use clap_num::number_range;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct ProgramArgs {
    /// Destination folder to store scraped Gelbooru images
    #[arg(short, long, required = true)]
    destination: PathBuf,

    /// Tags to search f or at Gelbooru
    #[arg(num_args(0..), short, long, required = true)]
    tags: Vec<String>,

    /// How many entries it should scrape for
    #[arg(short, long, default_value_t = 100, value_parser = leq_100)]
    limit: u8
}

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ProgramArgs { destination, tags, limit } = ProgramArgs::parse();
    if !destination.is_dir() {
        return Err("Please specify a valid destination folder.".into());
    }

    let client = GClient::public();

    let mut request = posts();
    for tag in tags { request = request.tag(tag); }

    println!("Fetching Information.. ⏳");
    let PostQuery { posts, .. } = request
        .limit(limit as usize)
        .send(&client).await?;
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots().https_only()
        .enable_http1().build();
    let http_client = hyper::Client::builder().build::<_, hyper::Body>(connector);

    let length = posts.len();
    for (idx, post) in posts.into_iter().enumerate() {
        tokio_scoped::scope(|scope| {
            print!("[{idx} of {length}] ", idx = idx+1);
            scope.spawn(async {
                deploy(
                    http_client.clone(),
                    post,
                    destination.to_str().unwrap()
                ).await
                    .unwrap()
            });
        });
    }

    Ok(println!(
        "Downloaded at folder `{destination}`. ✅",
        destination = destination.to_str().unwrap()
    ))
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

fn leq_100(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
