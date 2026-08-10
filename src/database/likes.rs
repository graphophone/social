use crate::database;

pub trait LikesDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()>;
    async fn dislike_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()>;
    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i16, page_size: i16) -> anyhow::Result<Vec<i64>>;
    async fn get_likes_count_for_track(&self, track_id: i64) -> anyhow::Result<i64>;
}

impl LikesDb for database::SocialDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO likes (user_id, track_id) VALUES ($1, $2);", user_id, track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn dislike_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()> {
        sqlx::query!(r"
            DELETE FROM likes
            WHERE user_id = $1 AND track_id = $2;
        ", user_id, track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i16, page_size: i16) -> anyhow::Result<Vec<i64>> {
        let offset = (page_number - 1) * page_size;
        let tracks_ids = sqlx::query_scalar!(r"
            SELECT track_id FROM likes
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        ", user_id, page_size as i64, offset as i64)
            .fetch_all(&self.pool)
            .await?;
        Ok(tracks_ids)
    }

    async fn get_likes_count_for_track(&self, track_id: i64) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar!(r"
            SELECT COUNT(*)
            FROM likes
            WHERE track_id = $1;
        ", track_id)
            .fetch_one(&self.pool)
            .await?;
        match count {
            Some(count) => Ok(count),
            None => Err(sqlx::Error::RowNotFound.into()),
        }
    }
}