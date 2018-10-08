use crate::{
  LodestoneApi,
  builder::Builder,
  models::id::FreeCompanyId,
};

use lodestone_parser::models::free_company::FreeCompany;

use std::borrow::Cow;

/// A builder for fetching free company information from XIVAPI.
#[derive(Debug, Serialize)]
pub struct FreeCompanyBuilder<'x> {
  #[serde(skip)]
  api: &'x LodestoneApi,

  #[serde(skip)]
  id: FreeCompanyId,
}

impl Builder<'x> for FreeCompanyBuilder<'x> {
  type Output = FreeCompany;

  fn api(&self) -> &'x LodestoneApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/free_company/{}", self.id.0))
  }
}

impl FreeCompanyBuilder<'x> {
  crate fn new(api: &'x LodestoneApi, id: FreeCompanyId) -> Self {
    FreeCompanyBuilder {
      api,
      id,
    }
  }
}

