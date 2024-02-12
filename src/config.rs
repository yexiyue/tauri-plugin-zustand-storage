use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
   pub path: String,
}
