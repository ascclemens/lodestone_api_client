# lodestone_api_client

It's really basic!

```rust
use lodestone_api::prelude::*;

fn main() -> Result<()> {
  let api = Lodestone::default();

  let search = api
    .character_search()
    .name("Xenosys Vex")
    .send()?;

  println!("{:#?}", search.results);

  Ok(())
}
```
