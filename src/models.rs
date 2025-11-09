// https://derpibooru.org/pages/api
// https://github.com/fox0/derpibooru-proxy/blob/master/src/models.rs
// https://transform.tools/json-to-rust-serde

use std::ops::{Add, AddAssign, Mul, Sub};

use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};

use crate::config::CONFIG;

/// This is a list of general parameters that are useful when working with the API.
/// Not all parameters may be used in every request.
#[derive(Debug, Default, Serialize)]
pub struct Parameters<'a> {
    /// Assuming the user can access the filter ID given by the parameter,
    /// overrides the current filter for this request.
    /// This is primarily useful for unauthenticated API access.
    pub filter_id: Option<u32>,
    /// An optional authentication token.
    /// If omitted, no user will be authenticated.
    pub key: Key<'a>,
    /// Controls the current page of the response, if the response is paginated.
    /// Empty values default to the first page.
    pub page: Page,
    /// Controls the number of results per page, up to a limit of 50, if the response is paginated.
    /// The default is 25.
    pub per_page: PerPage,
    /// The current search query, if the request is a search request.
    pub q: Option<String>,
    /// The current sort field, if the request is a search request.
    pub sf: Option<SortField>,
    /// The current sort direction, if the request is a search request.
    pub sd: Option<SortDirection>,
}

/// An optional authentication token.
#[derive(Debug, Serialize)]
pub struct Key<'a>(&'a Option<String>);

impl Default for Key<'_> {
    fn default() -> Self {
        Self(&CONFIG.api_key)
    }
}

/// Controls the current page of the response, if the response is paginated.
/// Empty values default to the first page.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Page(usize);

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}

impl PartialOrd<usize> for Page {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl PartialEq<usize> for Page {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl Sub<usize> for Page {
    type Output = usize;

    fn sub(self, rhs: usize) -> Self::Output {
        self.0 - rhs
    }
}

impl Add<usize> for Page {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        self.0 + rhs
    }
}

impl AddAssign<usize> for Page {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

/// Controls the number of results per page, up to a limit of 50, if the response is paginated.
/// The default is 25.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct PerPage(usize);

impl PerPage {
    pub const MAX: Self = Self(50);
}

impl Default for PerPage {
    fn default() -> Self {
        Self(25)
    }
}

impl Mul<PerPage> for Page {
    type Output = usize;

    fn mul(self, rhs: PerPage) -> Self::Output {
        self.0 * rhs.0
    }
}

impl Mul<PerPage> for usize {
    type Output = usize;

    fn mul(self, rhs: PerPage) -> Self::Output {
        self * rhs.0
    }
}

#[derive(Debug)]
pub struct PerPageError;

impl std::fmt::Display for PerPageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PerPageError")
    }
}

impl TryFrom<usize> for PerPage {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (1..=50).contains(&value) {
            Ok(Self(value))
        } else {
            Err(anyhow!(PerPageError))
        }
    }
}

impl From<PerPage> for usize {
    fn from(value: PerPage) -> Self {
        value.0
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    // ("id", "Sort by image ID"),
    // ("updated_at", "Sort by last modification date"),
    // ("first_seen_at", "Sort by initial post date"),
    // ("aspect_ratio", "Sort by aspect ratio"),
    // ("faves", "Sort by fave count"),
    // ("upvotes", "Sort by upvotes"),
    // ("downvotes", "Sort by downvotes"),
    // ("score", "Sort by score"),
    /// Sort by Wilson score
    WilsonScore,
    // ("_score", "Sort by relevance"),
    // ("width", "Sort by width"),
    // ("height", "Sort by height"),
    // ("comment_count", "Sort by comments"),
    // ("tag_count", "Sort by tag count"),
    // ("pixels", "Sort by pixels"),
    // ("size", "Sort by file size"),
    // ("duration", "Sort by duration"),
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

// https://derpibooru.org/api/v1/json/search/images?q=safe
#[derive(Debug, Deserialize)]
pub struct SearchImages {
    pub total: usize,
    pub images: Vec<Image>,
    // pub interactions: Vec<Value>,
}

// #[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    pub representations: Representations,
    // pub spoilered: bool,
    // pub thumbnails_generated: bool,
    // pub score: i64,
    // // pub intensities: Option<Intensities>,
    // pub animated: bool,
    // pub sha512_hash: String,
    // pub source_urls: Vec<String>,
    // pub width: i64,
    // pub size: i64,
    // pub orig_size: i64,
    // // pub duplicate_of: Value,
    // pub first_seen_at: String,
    // pub processed: bool,
    // pub source_url: String,
    // pub updated_at: String,
    // pub tag_ids: Vec<i64>,
    // pub view_url: String,
    // pub description: String,
    // pub mime_type: String,
    // pub duration: f64,
    // pub comment_count: i64,
    // pub aspect_ratio: f64,
    // pub upvotes: i64,
    // pub hidden_from_users: bool,
    // pub format: String,
    // pub id: i64,
    // pub name: String,
    // pub tag_count: i64,
    // pub uploader_id: Option<i64>,
    // pub uploader: Option<String>,
    // pub faves: i64,
    // pub downvotes: i64,
    // pub wilson_score: f64,
    // pub tags: Vec<String>,
    // // pub orig_sha512_hash: Option<String>,
    // pub height: i64,
    // pub created_at: String,
    // pub deletion_reason: Value,
}

// #[derive(Debug, Deserialize)]
// pub struct Intensities {
//     pub nw: f64,
//     pub ne: f64,
//     pub sw: f64,
//     pub se: f64,
// }

// #[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Representations {
    pub full: String,
    // pub small: String,
    // pub thumb_tiny: String,
    // pub thumb_small: String,
    // pub thumb: String,
    // pub medium: String,
    // pub large: String,
    // pub tall: String,
}
