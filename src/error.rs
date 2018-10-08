pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "could not url encode params: {}", _0)]
  UrlEncode(#[cause] serde_urlencoded::ser::Error),
  #[fail(display = "could not process request: {}", _0)]
  Reqwest(#[cause] reqwest::Error),
  #[fail(display = "could not parse json: {}", _0)]
  Json(#[cause] serde_json::Error),
}
