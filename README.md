# figment_string
Forces data parsed by a [`figment::Provider`](https://docs.rs/figment/latest/figment/trait.Provider.html) to be parsed as a string. See [this GitHub issue](https://github.com/SergioBenitez/Figment/issues/42) and the [`figment::Providers::Env`](https://docs.rs/figment/latest/figment/providers/struct.Env.html) documentation for some background.

Ever see this error?
```plain
% env NAME=8080 cargo run
Error: invalid type: found unsigned int `8080`, expected a string for key "NAME"
 in environment variable(s)
```

Well, now you can do this, and it will work even with numbers or booleans or whatever as input:

```rust
# use figment::{Figment, providers::Env};
# use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
   #[serde(deserialize_with = "figment_string::deserialize_as_string")]
   name: String,
}

fn main() {
    temp_env::with_var("NAME", Some("8080"), || {
        let config: Config = Figment::new()
            .merge(figment::providers::Env::raw())
            .extract()
            .unwrap();
        println!("Hello, {}!", config.name);
    });
}
```
