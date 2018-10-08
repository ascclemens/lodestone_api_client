#![feature(never_type)]

use lodestone_api_client::prelude::*;

fn main() -> Result<(), failure::Error> {
  let api = LodestoneApi::default();

  let res = api
    .character_search()
    .name("Duvivi Duvi")
    .world(World::Adamantoise)
    .send()?;

  // let id = res.characters[0].id;

  // let res: CharInfoResult = api
  //   .character(1)
  //   .columns(&["Name", "Server", "Race", "Gender"])
  //   .json()?;

  // let res = api
  //   .character(1.into())
  //   .send()?;

  // let res = api.enemy(7537.into()).send()?;
  // let res = api.character(2.into()).send()?;
  // let res = api.free_company_search().name("a").server(World::Adamantoise).send();
  // let res = api.free_company(9233645873504730768.into()).send();
  // let res = api.free_company(9233645873504776755.into()).send();
  // let res = api.linkshell_search()
  //   .name("lala world")
  //   .server(World::Adamantoise)
  //   .send();
  // let res = api.linkshell(20547673299957974.into()).send();

  println!("{:#?}", res);

  Ok(())
}
