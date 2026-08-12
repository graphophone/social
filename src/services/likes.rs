use tonic::{Request, Response, Status};

pub mod likes {
    tonic::include_proto!("likes");
}

use likes::{likes_server::{Likes}, Empty, LikeRequest, DislikeRequest, GetLikedTracksRequest, GetLikesCountRequest};

use crate::{database::{SocialDb, likes::LikesDb}, services::likes::likes::{LikedTracksResponse, LikesCountResponse}};

pub struct LikesService {
    likes_db: SocialDb,
}

#[tonic::async_trait]
impl Likes for LikesService {
    async fn like_track(&self, req: Request<LikeRequest>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.like_track(req.user_id, req.track_id).await;
        match res {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn dislike_track(&self, req: Request<DislikeRequest>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.dislike_track(req.user_id, req.track_id).await;
        match res {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn get_likes_count(&self, req: Request<GetLikesCountRequest>) -> Result<Response<LikesCountResponse>, Status> {
        let req = req.into_inner();
        let res = self.likes_db.get_likes_count_for_track(req.track_id).await;
        match res {
            Ok(count) => Ok(Response::new(LikesCountResponse { count })),
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }

    async fn get_liked_tracks(&self, req: Request<GetLikedTracksRequest>) -> Result<Response<LikedTracksResponse>, Status> {
        let req = req.into_inner();
        let tracks_query = self.likes_db.get_liked_tracks_for_user(
            req.user_id, req.page_number, req.page_size
        );
        let count_query = self.likes_db.get_liked_tracks_count_for_user(req.user_id);
        let (tracks_res, count_res) = tokio::join!(tracks_query, count_query);
        let track_ids = match tracks_res {
            Ok(track_ids) => track_ids,
            Err(e) => return Err(Status::from_error(Box::new(e))),
        };
        let count = match count_res {
            Ok(count) => count,
            Err(e) => return Err(Status::from_error(Box::new(e))),
        };
        Ok(Response::new(LikedTracksResponse { track_ids, total_count: count }))
    }
}