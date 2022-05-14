mod img;
use std::env;

use axum::{
    routing::post, 
    extract::Multipart,
    Router, Extension
};

use crate::img::{Image, upload_image};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let app = Router::new()
                        .route("/upload", post(upload))
                        .layer(&Extension(args));

    // run it with hyper on localhost:2000
    axum::Server::bind(&"0.0.0.0:2000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn upload(mut picture_data: Multipart, Extension(args): Extension<Vec<String>>) {
    while let Some(field) = picture_data.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("field name: {:?}", name);

        match &*name {
            "picture" => {
                let img = Image::new_from_bytes(data.to_vec());
                upload_image(
                    &args[1],
                    &args[2],
                    "rustgallery",
                    "image",
                    &img
                ).await;
            },
            "visibility" => {

            },
            _ => ()
        }
    }
}   