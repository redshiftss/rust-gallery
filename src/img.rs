extern crate s3;
use std::fs::File;
use std::{io, str};
use std::io::{Read, Write};
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
pub struct Image {
    bytes: Vec<u8>
}

impl Image {
    pub fn new(name : &str) -> Result<Self, io::Error>{
        let mut v = Vec::new();
        let mut i = File::open(name)?;
        i.read_to_end(&mut v)?;

        Ok(Self{
            bytes: v
        })
    }

    pub fn new_from_bytes(bs: Vec<u8>) -> Self {
        Self{bytes: bs}
    }
}

// struct BinaryImage {
//     bytes: Vec<u8>
// }

// impl From<Image> for BinaryImage{
//     fn from(_: Image) -> Self {
//         Self{
//             bytes: Image.to_vec
//         }
//     }
// }


pub async fn try_image() -> Result<()> {
    let i = Image::new("./wooloo.png").expect("Invalid path!");

    upload_image(
        "AKIAIOSFODNN7EXAMPLE",
        "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        "rustgallery",
        "wooloo",
        &i
    ).await?;

    let data = download_image(        "AKIAIOSFODNN7EXAMPLE",
                                      "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                                      "rustgallery",
                                      "wooloo").await?;
    println!("{:?}", data);
    // let mut buffer = File::create("foo.png")?;
    // buffer.write(&data)?;

    Ok(())
}

pub async fn upload_image(access_key: &str, secret_key: &str, bucket_name: &str, key: &str, file: &Image) -> Result<()> {
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
    let (_, code) = bucket.put_object(key, &file.bytes).await?;
    assert_eq!(200, code);

    Ok(())
}

pub async fn download_image(access_key: &str, secret_key: &str, bucket_name: &str, key: &str) -> Result<Vec<u8>> {
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
    let (data, code) = bucket.get_object(key).await?;
    assert_eq!(200, code);

    Ok(data)
}
