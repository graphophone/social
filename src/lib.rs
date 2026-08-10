pub mod config;
mod database;
use anyhow::Result;
use tokio::time;

pub async fn run(conf: config::Config) -> Result<()> {
    let db = database::SocialDb::build(&conf.database)
        .await?;
    db.ping()
        .await
        .expect("failed to ping database");
    println!("connected to database");

    time::sleep(time::Duration::from_secs(5))
        .await;
    println!("stopping social service");
    Ok(())
}