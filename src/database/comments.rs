use sqlx::types::time;

use crate::database::SocialDb;

pub trait CommentsDb {
    async fn add_comment(
        &self,
        user_id: i64,
        music_id: i64,
        content: String,
    ) -> sqlx::Result<CommentEntry>;
    async fn remove_comment(&self, comment_id: i64) -> sqlx::Result<()>;
    async fn update_comment(&self, comment_id: i64, new_content: String) -> sqlx::Result<()>;
    async fn check_owner(&self, comment_id: i64, user_id: i64) -> sqlx::Result<bool>;
    async fn toggle_comment_like(&self, comment_id: i64, user_id: i64) -> sqlx::Result<()>;
    async fn get_comments(
        &self,
        music_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<CommentEntry>)>;
}

impl CommentsDb for SocialDb {
    async fn add_comment(
        &self,
        user_id: i64,
        music_id: i64,
        content: String,
    ) -> sqlx::Result<CommentEntry> {
        let res = sqlx::query!(
            r"
            INSERT INTO comments (commenter_id, music_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, commenter_id, content, created_at;",
            user_id,
            music_id,
            content,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CommentEntry {
            comment_id: res.id,
            commenter_id: res.commenter_id,
            content: res.content,
            created_at: res.created_at,
            likes_count: 0,
            is_liked: false,
        })
    }

    async fn remove_comment(&self, comment_id: i64) -> sqlx::Result<()> {
        let res = sqlx::query!(
            r"
            DELETE FROM comments WHERE id = $1;",
            comment_id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    async fn check_owner(&self, comment_id: i64, user_id: i64) -> sqlx::Result<bool> {
        let res = sqlx::query_scalar!(
            r"
            SELECT EXISTS (
                SELECT 1
                FROM comments
                WHERE id = $1 AND commenter_id = $2
            );",
            comment_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(res.unwrap_or(false))
    }

    async fn toggle_comment_like(&self, comment_id: i64, user_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            r"
            WITH cte AS (
                DELETE FROM comment_likes
                WHERE comment_id = $1 AND user_id = $2
                RETURNING 1
            )
            INSERT INTO comment_likes (user_id, comment_id)
            SELECT $1, $2
            WHERE NOT EXISTS (SELECT 1 FROM cte)
            ON CONFLICT (user_id, comment_id) DO NOTHING
        ",
            user_id,
            comment_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_comment(&self, comment_id: i64, new_content: String) -> sqlx::Result<()> {
        sqlx::query!(
            r"
            UPDATE comments SET content = $2
            WHERE id = $1;
        ",
            comment_id,
            new_content
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_comments(
        &self,
        music_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<CommentEntry>)> {
        let mut tx = self.pool.begin().await?;

        let res = sqlx::query!(
            r"
            SELECT
                C.id,
                C.commenter_id,
                C.content,
                C.created_at,
                COUNT(CL.user_id) likes_count,
                (SELECT EXISTS (
                    SELECT 1 FROM comment_likes
                    WHERE comment_id = C.id AND user_id = $3)) is_liked
            FROM comments C
            JOIN comment_likes CL
            ON C.id = CL.comment_id
            WHERE C.music_id = $1
            GROUP BY C.id
            ORDER BY C.created_at DESC
            LIMIT 32 OFFSET $2;",
            music_id,
            (page_number - 1) * 32,
            requester_id,
        )
        .fetch_all(&mut *tx)
        .await?;

        let total_count = sqlx::query_scalar!(
            r"
            SELECT COUNT(*)
            FROM comments
            WHERE music_id = $1;",
            music_id
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap_or(0);

        let total_pages = ((total_count as f64) / 10.0).ceil() as i64;

        tx.commit().await?;

        Ok((
            total_pages,
            res.into_iter()
                .map(|v| CommentEntry {
                    comment_id: v.id,
                    commenter_id: v.commenter_id,
                    content: v.content,
                    created_at: v.created_at,
                    likes_count: v.likes_count.unwrap_or(0),
                    is_liked: v.is_liked.unwrap_or(false),
                })
                .collect(),
        ))
    }
}

pub struct CommentEntry {
    pub comment_id: i64,
    pub commenter_id: i64,
    pub content: String,
    pub created_at: time::OffsetDateTime,
    pub likes_count: i64,
    pub is_liked: bool,
}
