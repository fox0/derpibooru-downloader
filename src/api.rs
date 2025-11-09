use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use const_format::concatcp;
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::models::{Image, Parameters, PerPage, SearchImages};

const BASE_URL: &str = "https://derpibooru.org/api/v1/json";

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("http builder error")
});

// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html#consume-a-paginated-restful-api

pub struct ApiSearchImages<'a> {
    params: Parameters<'a>,
    images: <Vec<Image> as IntoIterator>::IntoIter,
    total: usize,
}

impl<'a> ApiSearchImages<'a> {
    const URL: &'static str = concatcp!(BASE_URL, "/search/images");

    pub fn new(mut params: Parameters<'a>) -> ApiSearchImages<'a> {
        params.per_page = PerPage::MAX;

        Self {
            params,
            images: vec![].into_iter(),
            total: 0,
        }
    }

    fn try_next(&mut self) -> anyhow::Result<Option<Image>> {
        if let Some(i) = self.images.next() {
            return Ok(Some(i));
        }

        if self.total != 0 && (self.params.page - 1) * self.params.per_page >= self.total {
            return Ok(None);
        }

        // Rate Limits 20 requests per 10 seconds
        thread::sleep(Duration::from_millis(250));

        let request = HTTP_CLIENT.get(Self::URL).query(&self.params).build()?;
        log::info!("{} {}", &request.method(), &request.url());

        let response = HTTP_CLIENT.execute(request)?;
        log::info!("{}", response.status());
        assert!(response.status() == StatusCode::OK);

        let r: SearchImages = if cfg!(debug_assertions) {
            log::warn!("DEBUG!");
            let rrr = response.text()?;
            // log::debug!("<<< {}", rrr);
            let jd = &mut serde_json::Deserializer::from_str(&rrr);
            match serde_path_to_error::deserialize(jd) {
                Ok(v) => v,
                Err(err) => {
                    let path = err.path().to_string();
                    log::error!("{}", path);
                    todo!();
                }
            }
        } else {
            response.json()?
        };

        self.params.page += 1;
        self.images = r.images.into_iter();
        self.total = r.total;
        log::debug!("total={}", r.total);

        Ok(self.images.next())
    }
}

impl Iterator for ApiSearchImages<'_> {
    type Item = anyhow::Result<Image>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(i)) => Some(Ok(i)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

// pub fn search_images(mut params: Parameters) -> anyhow::Result<Vec<Image>> {
//     const URL: &str = concatcp!(BASE_URL, "/search/images");
//     params.per_page = 50.try_into()?;

//     let mut result = vec![];
//     loop {
//         let request = HTTP_CLIENT.get(URL).query(&params).build()?;
//         log::info!("{} {}", &request.method(), &request.url());

//         let response = HTTP_CLIENT.execute(request)?;
//         log::info!("{}", response.status());
//         assert!(response.status() == StatusCode::OK);

//         let mut r: SearchImages = if cfg!(debug_assertions) {
//             log::warn!("DEBUG!");
//             let rrr = response.text()?;
//             // log::debug!("<<< {}", rrr);
//             let jd = &mut serde_json::Deserializer::from_str(&rrr);
//             match serde_path_to_error::deserialize(jd) {
//                 Ok(v) => v,
//                 Err(err) => {
//                     let path = err.path().to_string();
//                     log::error!("{}", path);
//                     todo!();
//                 }
//             }
//         } else {
//             response.json()?
//         };

//         let len = r.images.len();
//         result.append(&mut r.images);
//         log::info!("{} of {}", result.len(), r.total);
//         if len < usize::from(params.per_page) {
//             break;
//         }
//         params.page += 1;
//         thread::sleep(Duration::from_millis(250));
//     }

//     Ok(result)
// }
