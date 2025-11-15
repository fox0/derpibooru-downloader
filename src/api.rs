use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use const_format::concatcp;
use reqwest::StatusCode;
use reqwest::blocking::Client;
// use reqwest::header::{
//     ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, COOKIE, HeaderMap, HeaderValue,
//     UPGRADE_INSECURE_REQUESTS, USER_AGENT,
// };

// use crate::config::CONFIG;
use crate::models::{Image, Parameters, PerPage, SearchImages};

const BASE_URL: &str = "https://derpibooru.org/api/v1/json";

#[allow(clippy::expect_used)]
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    // let mut headers = HeaderMap::new();
    // headers.insert(ACCEPT, HeaderValue::from_static(&CONFIG.http.accept));
    // headers.insert(
    //     ACCEPT_LANGUAGE,
    //     HeaderValue::from_static(&CONFIG.http.accept_language),
    // );
    // headers.insert(
    //     CACHE_CONTROL,
    //     HeaderValue::from_static(&CONFIG.http.cache_control),
    // );
    // headers.insert(COOKIE, HeaderValue::from_static(&CONFIG.http.cookie));
    // headers.insert("priority", HeaderValue::from_static(&CONFIG.http.priority));
    // // REFERER
    // headers.insert(
    //     "sec-ch-ua",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua),
    // );
    // headers.insert(
    //     "sec-ch-ua-arch",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_arch),
    // );
    // headers.insert(
    //     "sec-ch-ua-bitness",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_bitness),
    // );
    // headers.insert(
    //     "sec-ch-ua-full-version",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_full_version),
    // );
    // headers.insert(
    //     "sec-ch-ua-full-version-list",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_full_version_list),
    // );
    // headers.insert(
    //     "sec-ch-ua-mobile",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_mobile),
    // );
    // headers.insert(
    //     "sec-ch-ua-model",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_model),
    // );
    // headers.insert(
    //     "sec-ch-ua-platform",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_platform),
    // );
    // headers.insert(
    //     "sec-ch-ua-platform-version",
    //     HeaderValue::from_static(&CONFIG.http.sec_ch_ua_platform_version),
    // );
    // headers.insert(
    //     "sec-fetch-dest",
    //     HeaderValue::from_static(&CONFIG.http.sec_fetch_dest),
    // );
    // headers.insert(
    //     "sec-fetch-mode",
    //     HeaderValue::from_static(&CONFIG.http.sec_fetch_mode),
    // );
    // headers.insert(
    //     "sec-fetch-site",
    //     HeaderValue::from_static(&CONFIG.http.sec_fetch_site),
    // );
    // headers.insert(
    //     "sec-fetch-user",
    //     HeaderValue::from_static(&CONFIG.http.sec_fetch_user),
    // );
    // headers.insert(
    //     UPGRADE_INSECURE_REQUESTS,
    //     HeaderValue::from_static(&CONFIG.http.upgrade_insecure_requests),
    // );
    // headers.insert(
    //     USER_AGENT,
    //     HeaderValue::from_static(&CONFIG.http.user_agent),
    // );
    // headers.insert(
    //     CONTENT_TYPE,
    //     HeaderValue::from_static("application/json;charset=utf-8"),
    // );
    // dbg!(&headers);

    Client::builder()
        // .default_headers(headers)
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

        // https://derpibooru.org/forums/meta/topics/site-development-notification-and-feedback-thread?post_id=5775592#post_5775592

        // Due to recent attacks, we have changed API search rate limits.
        // Before: 20 requests per 10 seconds
        // After: 10 requests per 10 seconds
        // Sorry for any inconvenience. Please adjust your API clients/bots accordingly.
        // Note: this does not affect non-API clients, so if you’re just browsing the site, you can safely disregard this notice
        thread::sleep(Duration::from_millis(1100));

        let request = HTTP_CLIENT.get(Self::URL).query(&self.params).build()?;
        log::info!("{} {}", &request.method(), &request.url());

        let response = HTTP_CLIENT.execute(request)?;
        log::info!("{}", response.status());
        assert!(response.status() == StatusCode::OK);

        let r: SearchImages = {
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
