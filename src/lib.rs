#![feature(in_band_lifetimes, never_type, crate_visibility_modifier)]

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use reqwest::Client;

use url::Url;

use std::str::FromStr;

pub mod builder;
pub mod error;
pub mod models;
pub mod prelude;
pub mod routes;
pub mod util;

use crate::{
  models::id::*,
  routes::{
    character::CharacterBuilder,
    free_company::FreeCompanyBuilder,
    search::{
      character::SearchBuilder as CharacterSearchBuilder,
      free_company::SearchBuilder as FreeCompanySearchBuilder,
    },
  }
};

/// The main driver for accessing XIVAPI.
#[derive(Debug)]
pub struct LodestoneApi {
  client: Client,
}

impl Default for LodestoneApi {
  fn default() -> Self {
    LodestoneApi::new()
  }
}

impl LodestoneApi {
  /// Create an API driver without an API key.
  pub fn new() -> Self {
    let client = Client::new();
    LodestoneApi { client }
  }

  crate fn url(&self, path: &str) -> Url {
    Url::from_str("https://lodestone.info").unwrap().join(path).unwrap()
  }

  /// Search for a character.
  pub fn character_search(&self) -> CharacterSearchBuilder {
    CharacterSearchBuilder::new(self)
  }

  /// Fetch a specific character by their Lodestone ID.
  pub fn character(&self, id: CharacterId) -> CharacterBuilder {
    CharacterBuilder::new(self, id)
  }

  /// Search for a free company.
  pub fn free_company_search(&self) -> FreeCompanySearchBuilder {
    FreeCompanySearchBuilder::new(self)
  }

  /// Fetch a specific free company by its ID.
  pub fn free_company(&self, id: FreeCompanyId) -> FreeCompanyBuilder {
    FreeCompanyBuilder::new(self, id)
  }
}
