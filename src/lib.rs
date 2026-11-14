pub mod config;
use std::sync::Arc;

use anyhow::Result;
use tonic::transport::Server;

use crate::services::followers::{FollowersService, followers::followers_server::FollowersServer};

mod database;
mod services;

pub async fn run(conf: config::Config) -> Result<()> {
    let addr = "0.0.0.0:8080".parse()?;

    let social_db = Arc::new(database::SocialDb::build(&conf.database).await?);

    let followers_service = FollowersService::new(Arc::clone(&social_db));

    println!("starting social service");
    Server::builder()
        .add_service(FollowersServer::new(followers_service))
        .serve(addr)
        .await?;

    println!("stopping social service");
    Ok(())
}
