pub mod config;
use anyhow::Result;
use tonic::transport::Server;

use crate::services::likes::{LikesServer, LikesService};

mod database;
mod services;

pub async fn run(conf: config::Config) -> Result<()> {
    let addr = "0.0.0.0:8080".parse()?;

    let db = database::SocialDb::build(&conf.database)
        .await?;
    db.ping()
        .await
        .expect("failed to ping database");
    let likes_service = LikesService::new(db);

    println!("starting social service");
    Server::builder()
        .add_service(LikesServer::new(likes_service))
        .serve(addr)
        .await?;

    println!("stopping social service");
    Ok(())
}