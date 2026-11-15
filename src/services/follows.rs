use std::sync::Arc;

use tonic::{Request, Response, Status, async_trait};

use crate::{
    database::{SocialDb, follows::FollowsDb},
    services::follows::follows::{
        Empty, FollowEntry, FollowsCount, FollowsPage, GetFollowsCountReq, GetFollowsReq,
        ToggleFollowReq,
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
    async fn get_follows_count(
        &self,
        req: Request<GetFollowsCountReq>,
    ) -> Result<Response<FollowsCount>, Status> {
        let req = req.into_inner();

        let res = self
            .social_db
            .get_follows_count(req.user_id, req.follower_id)
            .await
            .map_err(|_| Status::internal("failed to get user follows count"))?;

        Ok(Response::new(FollowsCount {
            followings_count: res.followings_count,
            followers_count: res.followers_count,
            is_followed: res.is_followed,
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

    async fn get_followers(
        &self,
        req: Request<GetFollowsReq>,
    ) -> Result<Response<FollowsPage>, Status> {
        let req = req.into_inner();

        let (total_pages, follows) = self
            .social_db
            .get_followers(req.user_id, req.page_number, req.requester_id)
            .await
            .map_err(|_| Status::internal("failed to get followers"))?;

        Ok(Response::new(FollowsPage {
            total_pages,
            follows: follows
                .into_iter()
                .map(|v| FollowEntry {
                    user_id: v.user_id,
                    follows_count: Some(FollowsCount {
                        followings_count: v.follows_count.followings_count,
                        followers_count: v.follows_count.followers_count,
                        is_followed: v.follows_count.is_followed,
                    }),
                })
                .collect(),
        }))
    }

    async fn get_followings(
        &self,
        req: Request<GetFollowsReq>,
    ) -> Result<Response<FollowsPage>, Status> {
        let req = req.into_inner();

        let (total_pages, follows) = self
            .social_db
            .get_followings(req.user_id, req.page_number, req.requester_id)
            .await
            .map_err(|_| Status::internal("failed to get followers"))?;

        Ok(Response::new(FollowsPage {
            total_pages,
            follows: follows
                .into_iter()
                .map(|v| FollowEntry {
                    user_id: v.user_id,
                    follows_count: Some(FollowsCount {
                        followings_count: v.follows_count.followings_count,
                        followers_count: v.follows_count.followers_count,
                        is_followed: v.follows_count.is_followed,
                    }),
                })
                .collect(),
        }))
    }
}
