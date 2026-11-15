pub mod config;
use std::sync::Arc;

use anyhow::Result;
use tonic::transport::Server;

use crate::services::{
    comments::{CommentsService, comments::comments_server::CommentsServer},
    follows::{FollowsService, follows::follows_server::FollowsServer},
};

mod database;
mod services;

pub async fn run(conf: config::Config) -> Result<()> {
    let addr = "0.0.0.0:8080".parse()?;

    let social_db = Arc::new(database::SocialDb::build(&conf.database).await?);

    let follows_service = FollowsService::new(Arc::clone(&social_db));
    let comments_service = CommentsService::new(Arc::clone(&social_db));

    println!("starting social service");
    Server::builder()
        .add_service(FollowsServer::new(follows_service))
        .add_service(CommentsServer::new(comments_service))
        .serve(addr)
        .await?;

    println!("stopping social service");
    Ok(())
}
