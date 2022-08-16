use super::config::MyConf;
use figment::{
    providers::{Format, Json},
    Figment,
};
use rocket::fairing::{AdHoc};
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
    pub bucket: Bucket,
}

impl Minio {
    pub async fn new(
        bucket_name: String,
        url: String,
        username: String,
        password: String,
    ) -> Result<Minio, S3Error> {
        let region = Region::Custom {
            region: "".into(),
            endpoint: url.clone(),
        };

        println!(
            "bucket_name:{} url:{} username:{} password:{}",
            bucket_name, url, username, password
        );

        let credentials = Credentials::new(
            Some(username.as_str()),
            Some(password.as_str()),
            None,
            None,
            None,
        )
        .unwrap();

        let _bucket = Bucket::new(bucket_name.as_str(), region, credentials)?.with_path_style();
        // let objs = _bucket.list("/".to_string(), Some("/".to_string())).await.unwrap();

        // println!("{:?}", objs);
        Ok(Minio { bucket: _bucket })
    }

    pub async fn get_object<T>(&self, path: T) -> Result<ResponseData, S3Error>
    where
        T: AsRef<str>,
    {
        // let l = self.bucket.list("/".to_string(), Some("/".to_string())).await.unwrap();
        // println!("{:?}", l);
        // self.bucket.get_object_stream(path, writer)?;
        let data = self.bucket.get_object(path).await?;
        Ok(data)
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("minio", |r| async {
        let conf: MyConf = Figment::new()
            .join(Json::file("config.json"))
            .extract()
            .expect("加载配置错误");

        let minio = Minio::new(
            conf.minio.bucket_name.clone(),
            conf.minio.endpoint.clone(),
            conf.minio.user.clone(),
            conf.minio.password.clone(),
        )
        .await
        .unwrap();

        r.manage(minio).manage(conf)
    })
}
