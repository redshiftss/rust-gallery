extern crate s3;
use image::{DynamicImage, ImageError, open};
use std::str;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use anyhow::Result;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let i = Image::new("./wooloo.png").expect("Invalid path!");

    upload_image(
        "AKIAIOSFODNN7EXAMPLE",
        "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        "rustgallery",
        "./wooloo.png"
    ).await?;

    Ok(())
}

async fn upload_image(access_key: &str, secret_key: &str, bucket_name: &str, file: &str) -> Result<()> {
    let minio = Storage {
        name: "minio".into(),
        region: Region::Custom {
            region: "".into(),
            endpoint: "http://127.0.0.1:9000".into(),
        },
        credentials: Credentials {
            access_key: Some(access_key.to_owned()),
            secret_key: Some(secret_key.to_owned()),
            security_token: None,
            session_token: None,
        },
        bucket: bucket_name.to_string(),
        location_supported: false,
    };

    let bucket = Bucket::new_with_path_style(&minio.bucket, minio.region, minio.credentials)?;
    let (_, code) = bucket.put_object("test_file", file.as_bytes()).await?;
    assert_eq!(200, code);

    Ok(())
}