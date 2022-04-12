use image::{DynamicImage, ImageError, open};
use bytes::Bytes;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error, Region, PKG_VERSION};

#[derive(Debug)]
struct Image {
    img: DynamicImage
}

impl Image {
    fn new(name : &str) -> Result<Self, ImageError>{
        let i = open(name)?;
        Ok(Self{
            img: i
        })
    }
}

fn main() {
    let i = Image::new("./wooloo.png").expect("Invalid path!");
}

async fn upload_image(client: &Client, bucket: &str, key: &str, img: &str) -> Result<(), Error> {
    let body = ByteStream::from_path(img).await;
    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
            println!("Successfully uploaded image!");
        }
        Err(e) => {
            println!("Error encountered: {}", e);
        }
    }
    Ok(())
}