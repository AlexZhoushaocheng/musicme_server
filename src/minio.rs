use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use s3::request_trait::ResponseData;

// pub async fn minio_test() {
//     let bucket_name = "mymusic";
//     let region = Region::Custom {
//         region: "".into(),
//         endpoint: "http://www.litesite.cn:9000".into(),
//     };
//     // let credentials = Credentials::default().unwrap();
//     let credentials = Credentials::new(Some("admin"), Some("admin1234"), None, None, None).unwrap();

//     let bucket = Bucket::new(bucket_name, region, credentials)
//         .unwrap()
//         .with_path_style();
// }


pub struct Minio {
    bucket: Bucket,
}

impl Minio {
    pub fn new(
        bucket_name: String,
        url: String,
        username: String,
        password: String,
    ) -> Result<Minio, S3Error> {
        let region = Region::Custom {
            region: "".into(),
            endpoint: url.clone(),
        };
        let credentials = Credentials::new(
            Some(username.as_str()),
            Some(password.as_str()),
            None,
            None,
            None,
        )
        .unwrap();

        let _bucket = Bucket::new(bucket_name.as_str(), region, credentials)?.with_path_style();
        
        Ok(Minio {
            bucket:_bucket
        })
    }

    pub async fn get_object<T>(self, path:T) -> Result<ResponseData, S3Error> where T:AsRef<str> {
        
        let data = self.bucket.get_object(path).await?;
        Ok(data)
    }
}
