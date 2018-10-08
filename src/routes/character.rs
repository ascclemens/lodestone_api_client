use crate::{
  LodestoneApi,
  builder::Builder,
  models::id::CharacterId,
};

use lodestone_parser::models::character::Character;

use std::borrow::Cow;

/// A builder for fetching character information from XIVAPI.
#[derive(Debug, Serialize)]
pub struct CharacterBuilder<'x> {
  #[serde(skip)]
  api: &'x LodestoneApi,

  #[serde(skip)]
  id: CharacterId,
}

impl Builder<'x> for CharacterBuilder<'x> {
  type Output = Character;

  fn api(&self) -> &'x LodestoneApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/character/{}", self.id.0))
  }
}

impl CharacterBuilder<'x> {
  crate fn new(api: &'x LodestoneApi, id: CharacterId) -> Self {
    CharacterBuilder {
      api,
      id,
    }
  }
}
