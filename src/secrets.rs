use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlueskySecrets {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Secrets {
    pub bsky: BlueskySecrets,
}

pub fn load_secrets() -> Secrets {
    // TODO: Don't unwrap
    let toml = std::fs::read_to_string("secrets.toml").unwrap();
    toml::from_str(&toml).unwrap()
}