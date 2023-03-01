#[macro_use]
extern crate lazy_static;

use std::io::{BufReader, BufWriter, Cursor};
use std::net::SocketAddr;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use image::ImageFormat;
use serde::Deserialize;
use tokio::time::Instant;

#[derive(Deserialize, Debug)]
enum Mode {
    #[serde(rename = "resize")]
    Resize,
    #[serde(rename = "crop")]
    Crop,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Resize
    }
}

#[derive(Deserialize, Debug)]
pub struct Params {
    image: String,
    width: u32,
    height: u32,
    #[serde(default)]
    mode: Mode,
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .http2_prior_knowledge()
        .gzip(true)
        .build()
        .unwrap();
}

async fn get_remote_image(image_url: &str) -> reqwest::Response {
    CLIENT.get(image_url).send().await.unwrap()
}

fn get_mimetype(headers: &reqwest::header::HeaderMap) -> &str {
    headers
        .get("Content-Type")
        .map(|mime| mime.to_str().unwrap())
        .unwrap()
}

async fn handle_image_processing(params: Query<Params>) -> impl IntoResponse {
    let remote_image = get_remote_image(&params.image).await;

    let mime_type = get_mimetype(remote_image.headers()).to_owned();

    let img = image::io::Reader::new(BufReader::new(Cursor::new(
        &remote_image.bytes().await.expect("Something went wrong"),
    )))
    .with_guessed_format()
    .expect("This fails, check why")
    .decode()
    .expect("Decode failed");

    let new_image = match params.mode {
        Mode::Resize => image::imageops::resize(
            &img,
            params.width,
            params.height,
            image::imageops::FilterType::Nearest,
        ),
        Mode::Crop => image::imageops::crop_imm(&img, 0, 0, params.width, params.height).to_image(),
    };

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    new_image
        .write_to(
            &mut buffer,
            ImageFormat::from_mime_type(&mime_type).unwrap(),
        )
        .unwrap();

    (
        [(axum::http::header::CONTENT_TYPE, mime_type)],
        [(axum::http::header::VARY, "Accept-Encoding")],
        buffer.into_inner().unwrap().into_inner(),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_image_processing));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
