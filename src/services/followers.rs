use std::sync::Arc;

use tonic::{Request, Response, Status, async_trait};

use crate::{
    database::{SocialDb, followers::FollowersDb},
    services::followers::followers::{Empty, FollowsCount, ToggleFollowReq, UserId},
};

pub mod followers {
    tonic::include_proto!("followers");
}

pub struct FollowersService {
    social_db: Arc<SocialDb>,
}

impl FollowersService {
    pub fn new(social_db: Arc<SocialDb>) -> Self {
        FollowersService { social_db }
    }
}

#[async_trait]
impl followers::followers_server::Followers for FollowersService {
    async fn get_user_follows_count(
        &self,
        req: Request<UserId>,
    ) -> Result<Response<FollowsCount>, Status> {
        let req = req.into_inner();

        let (following_count, followed_count) = self
            .social_db
            .get_user_follows_count(req.user_id)
            .await
            .map_err(|_| Status::internal("failed to get user followers count"))?;

        Ok(Response::new(FollowsCount {
            following_count,
            followed_count,
        }))
    }

    async fn toggle_follow(
        &self,
        req: Request<ToggleFollowReq>,
    ) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();

        self.social_db
            .toggle_follow(req.followee_id, req.follower_id)
            .await
            .map_err(|_| Status::internal("failed to toggle error"))?;

        Ok(Response::new(Empty {}))
    }
}
