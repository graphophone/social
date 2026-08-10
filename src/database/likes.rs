use crate::database;

pub trait LikesDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()>;
    async fn dislike_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()>;
    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i16, page_size: i16) -> anyhow::Result<Vec<i64>>;
    async fn get_likes_count_for_track(&self, track_id: i64) -> anyhow::Result<i64>;
}

impl LikesDb for database::SocialDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()> {
        let query = "INSERT INTO likes (user_id, track_id) VALUES ($1, $2);";
        sqlx::query(query)
            .bind(user_id)
            .bind(track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn dislike_track(&self, user_id: i64, track_id: i64) -> anyhow::Result<()> {
        let query = r"
            DELETE FROM likes
            WHERE user_id = $1 AND track_id = $1;
        ";
        sqlx::query(query)
            .bind(user_id)
            .bind(track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i16, page_size: i16) -> anyhow::Result<Vec<i64>> {
        let query = r"
            SELECT track_id FROM likes
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        ";
        let tracks_ids = sqlx::query_scalar::<_, i64>(query)
            .bind(user_id)
            .bind(page_size)
            .bind((page_number - 1) * page_size)
            .fetch_all(&self.pool)
            .await?;
        Ok(tracks_ids)
    }

    async fn get_likes_count_for_track(&self, track_id: i64) -> anyhow::Result<i64> {
        let query = r"
            SELECT COUNT(*)
            FROM likes
            WHERE track_id = $1;
        ";
        let count = sqlx::query_scalar::<_, i64>(query)
            .bind(track_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }
}