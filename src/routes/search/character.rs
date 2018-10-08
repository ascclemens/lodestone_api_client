use crate::{
  LodestoneApi,
  builder::Builder,
};

use lodestone_parser::models::search::{
  Paginated,
  character::CharacterSearchItem,
};

use ffxiv_types::World;

use std::borrow::Cow;

/// A builder for searching for a character on XIVAPI.
#[derive(Debug, Serialize)]
pub struct SearchBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x LodestoneApi,

  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  world: Option<World>,

  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<usize>,
}

impl Builder<'x> for SearchBuilder<'x, 'a> {
  type Output = Paginated<CharacterSearchItem>;

  fn api(&self) -> &'x LodestoneApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Borrowed("/character/search")
  }
}

impl SearchBuilder<'x, 'a> {
  crate fn new(api: &'x LodestoneApi) -> Self {
    SearchBuilder {
      api,
      name: None,
      world: None,
      page: None,
    }
  }

  /// Specify the name to search for.
  pub fn name(&mut self, n: &'a str) -> &mut Self {
    self.name = Some(n);
    self
  }

  /// Specify the world to search on.
  pub fn world(&mut self, s: World) -> &mut Self {
    self.world = Some(s);
    self
  }

  // Select the page of results to view.
  pub fn page(&mut self, p: usize) -> &mut Self {
    self.page = Some(p);
    self
  }
}
