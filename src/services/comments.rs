use std::sync::Arc;

use tonic::{Request, Response, Status, async_trait};

use crate::{
    database::{SocialDb, comments::CommentsDb},
    services::comments::comments::{
        AddCommentReq, CheckOwnerReq, CheckOwnerRes, CommentsPage, Empty, GetCommentsReq,
        RemoveCommentReq, ToggleLikeReq, UpdateCommentReq, comments_server::Comments,
    },
};

pub mod comments {
    tonic::include_proto!("comments");
}

pub struct CommentsService {
    social_db: Arc<SocialDb>,
}

impl CommentsService {
    pub fn new(social_db: Arc<SocialDb>) -> Self {
        CommentsService { social_db }
    }
}

#[async_trait]
impl Comments for CommentsService {
    async fn add_comment(&self, req: Request<AddCommentReq>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();

        self.social_db
            .add_comment(req.user_id, req.music_id, req.content)
            .await
            .map_err(|_| Status::internal("failed to add comment"))?;

        Ok(Response::new(Empty {}))
    }

    async fn remove_comment(
        &self,
        req: Request<RemoveCommentReq>,
    ) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();

        self.social_db
            .remove_comment(req.comment_id)
            .await
            .map_err(|_| Status::internal("failed to remove comment"))?;

        Ok(Response::new(Empty {}))
    }

    async fn update_comment(
        &self,
        req: Request<UpdateCommentReq>,
    ) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();

        self.social_db
            .update_comment(req.comment_id, req.new_content)
            .await
            .map_err(|_| Status::internal("failed to update comment"))?;

        Ok(Response::new(Empty {}))
    }

    async fn check_owner(
        &self,
        req: Request<CheckOwnerReq>,
    ) -> Result<Response<CheckOwnerRes>, Status> {
        let req = req.into_inner();

        let res = self
            .social_db
            .check_owner(req.comment_id, req.user_id)
            .await
            .map_err(|_| Status::internal("failed to check owner"))?;

        Ok(Response::new(CheckOwnerRes { is_owner: res }))
    }

    async fn toggle_like(&self, req: Request<ToggleLikeReq>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();

        self.social_db
            .toggle_comment_like(req.comment_id, req.user_id)
            .await
            .map_err(|_| Status::internal("failed to toggle comment like"))?;

        Ok(Response::new(Empty {}))
    }

    async fn get_comments(
        &self,
        req: Request<GetCommentsReq>,
    ) -> Result<Response<CommentsPage>, Status> {
        let req = req.into_inner();

        let (total_pages, comments) = self
            .social_db
            .get_comments(req.music_id, req.page_number, req.requester_id)
            .await
            .map_err(|_| Status::internal("failed to get comments"))?;

        Ok(Response::new(CommentsPage {
            total_pages,
            comments: comments
                .into_iter()
                .map(|v| comments::CommentEntry {
                    comment_id: v.comment_id,
                    commenter_id: v.commenter_id,
                    content: v.content,
                    created_at: v.created_at.unix_timestamp(),
                    likes_count: v.likes_count,
                    is_liked: v.is_liked,
                })
                .collect(),
        }))
    }
}
