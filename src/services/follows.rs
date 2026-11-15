use std::sync::Arc;

use tonic::{Request, Response, Status, async_trait};

use crate::{
    database::{SocialDb, follows::FollowsDb},
    services::follows::follows::{
        Empty, FollowsCount, FollowsPage, GetFollowsReq, ToggleFollowReq, UserId,
    },
};

pub mod follows {
    tonic::include_proto!("follows");
}

pub struct FollowsService {
    social_db: Arc<SocialDb>,
}

impl FollowsService {
    pub fn new(social_db: Arc<SocialDb>) -> Self {
        FollowsService { social_db }
    }
}

#[async_trait]
impl follows::follows_server::Follows for FollowsService {
    async fn get_user_follows_count(
        &self,
        req: Request<UserId>,
    ) -> Result<Response<FollowsCount>, Status> {
        let req = req.into_inner();

        let (followings_count, followers_count) = self
            .social_db
            .get_user_follows_count(req.user_id)
            .await
            .map_err(|_| Status::internal("failed to get user follows count"))?;

        Ok(Response::new(FollowsCount {
            followings_count,
            followers_count,
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

    async fn get_user_followers(
        &self,
        req: Request<GetFollowsReq>,
    ) -> Result<Response<FollowsPage>, Status> {
        let req = req.into_inner();

        let (total_pages, user_ids) = self
            .social_db
            .get_followers(req.user_id, req.page_number)
            .await
            .map_err(|_| Status::internal("failed to get followers"))?;

        Ok(Response::new(FollowsPage {
            total_pages,
            user_ids,
        }))
    }

    async fn get_user_followings(
        &self,
        req: Request<GetFollowsReq>,
    ) -> Result<Response<FollowsPage>, Status> {
        let req = req.into_inner();

        let (total_pages, user_ids) = self
            .social_db
            .get_followings(req.user_id, req.page_number)
            .await
            .map_err(|_| Status::internal("failed to get followings"))?;

        Ok(Response::new(FollowsPage {
            total_pages,
            user_ids,
        }))
    }
}
