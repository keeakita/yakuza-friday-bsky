use anyhow::Result;
use bsky_sdk::{
    agent::config::{Config, FileStore},
    BskyAgent,
};
use log::{debug, info, warn};

mod secrets;

const SESSION_FILE_PATH: &str = "session.json";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let secrets = secrets::load_secrets();

    let agent = match try_load_session().await {
        Ok(agent) => agent,
        Err(e) => {
            info!("Failed to load session, creating a new one: {e}");
            let agent = BskyAgent::builder().build().await?;
            agent
                .login(secrets.bsky.username, secrets.bsky.password)
                .await?;
            save_session(&agent).await?;
            agent
        }
    };

    let prefs = agent.get_preferences(false).await?;
    debug!("{:?}", prefs);

    Ok(())
}

async fn try_load_session() -> Result<BskyAgent> {
    let agent = BskyAgent::builder()
        .config(Config::load(&FileStore::new(SESSION_FILE_PATH)).await?)
        .build()
        .await?;
    let result = agent.api.com.atproto.server.get_session().await;
    assert!(result.is_ok());
    Ok(agent)
}

async fn save_session(agent: &BskyAgent) -> Result<()> {
    agent
        .to_config()
        .await
        .save(&FileStore::new(SESSION_FILE_PATH))
        .await?;
    Ok(())
}
