use tonic::{Request, Response, Status};

pub mod likes {
    tonic::include_proto!("likes");
}

use likes::{likes_server::Likes, Empty, LikeRequest, DislikeRequest, GetLikedpostsRequest, GetLikesCountRequest};
pub use likes::likes_server::LikesServer;
use crate::{database::{SocialDb, likes::LikesDb}, services::likes::likes::{LikedpostsResponse, LikesCountResponse}};

pub struct LikesService {
    likes_db: SocialDb,
}

impl LikesService {
    pub fn new(likes_db: SocialDb) -> LikesService {
        LikesService { likes_db }
    }
}

#[tonic::async_trait]
impl Likes for LikesService {
    async fn like_post(&self, req: Request<LikeRequest>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.like_post(req.user_id, req.post_id).await;
        match res {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn dislike_post(&self, req: Request<DislikeRequest>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.dislike_post(req.user_id, req.post_id).await;
        match res {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn get_likes_count(&self, req: Request<GetLikesCountRequest>) -> Result<Response<LikesCountResponse>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.get_likes_count_for_post(req.post_id).await;
        match res {
            Ok(count) => Ok(Response::new(LikesCountResponse { count })),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn get_liked_posts(&self, req: Request<GetLikedpostsRequest>) -> Result<Response<LikedpostsResponse>, Status> {
        let req = req.into_inner();
        let posts_query = self.likes_db.get_liked_posts_for_user(
            req.user_id, req.page_number, req.page_size
        );
        let count_query = self.likes_db.get_liked_posts_count_for_user(req.user_id);
        let (posts_res, count_res) = tokio::join!(posts_query, count_query);
        let post_ids = match posts_res {
            Ok(post_ids) => post_ids,
            Err(e) => return Err(Status::from_error(Box::new(e))),
        };
        let count = match count_res {
            Ok(count) => count,
            Err(e) => return Err(Status::from_error(Box::new(e))),
        };
        Ok(Response::new(LikedpostsResponse { post_ids, total_count: count }))
    }
}