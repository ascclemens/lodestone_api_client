use chrono::{DateTime, Utc};

pub mod id;

// FIXME: can't depend on lodestone_api because it pulls in so much crap (including ancient cookie
// with a security advisory), but shouldn't have to maintain these separately
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum RouteResult<T> {
  /// The resource was successfully retrieved from the database.
  Success {
    /// The resource
    result: T,
    /// The date at which the resource was last scraped and updated
    last_update: DateTime<Utc>,
  },
  /// The resource wasn't found, so it has been queued for scraping.
  Adding {
    /// The position the resource is in its scrape queue
    queue_position: u64,
  },
  /// The resource was scraped once and returned.
  Scraped {
    /// The resource
    result: T,
  },
  /// The resource was scraped and cached for a limited amount of time.
  Cached {
    /// The resource
    result: T,
    /// When the resource will expire from the cache, after which new requests will result in a new
    /// scrape
    expires: DateTime<Utc>,
  },
  /// The resource was not found.
  NotFound,
  /// An error ocurred when processing the route.
  Error {
    /// The error message
    error: String,
  },
}

pub use lodestone_parser::models::*;
