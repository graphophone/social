use crate::database;

pub trait LikesDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> Result<(), sqlx::Error>;
    async fn dislike_track(&self, user_id: i64, track_id: i64) -> Result<(), sqlx::Error>;
    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i32, page_size: i32) -> Result<Vec<i64>, sqlx::Error>;
    async fn get_likes_count_for_track(&self, track_id: i64) -> Result<i64, sqlx::Error>;
    async fn get_liked_tracks_count_for_user(&self, user_id: i64) -> Result<i64, sqlx::Error>;
}

impl LikesDb for database::SocialDb {
    async fn like_track(&self, user_id: i64, track_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO likes (user_id, track_id) VALUES ($1, $2);", user_id, track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn dislike_track(&self, user_id: i64, track_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r"
            DELETE FROM likes
            WHERE user_id = $1 AND track_id = $2;
        ", user_id, track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_liked_tracks_for_user(&self, user_id: i64, page_number: i32, page_size: i32) -> Result<Vec<i64>, sqlx::Error> {
        let offset: i32 = (page_number - 1) * page_size;
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

    async fn get_liked_tracks_count_for_user(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r"
            SELECT COUNT(*)
            FROM likes
            WHERE user_id = $1;
        ", user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(count.unwrap_or_default())
    }

    async fn get_likes_count_for_track(&self, track_id: i64) -> Result<i64, sqlx::Error> {
        let count: Option<i64> = sqlx::query_scalar!(r"
            SELECT COUNT(user_id)
            FROM likes
            WHERE track_id = $1;
        ", track_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(count.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
use crate::database::likes::LikesDb;
use super::database::SocialDb;

    #[sqlx::test]
    async fn test_like_track(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_track(1, 1)
            .await?;
        Ok(())
    }

    #[sqlx::test]
    async fn test_dislike_track(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.dislike_track(1, 1)
            .await?;
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_likes_count_all_likes(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        tokio::try_join!(
            db.like_track(1, 1),
            db.like_track(2, 1),
            db.like_track(3, 1),
            db.like_track(4, 1),
        )?;
        let count = db.get_likes_count_for_track(1).await?;
        assert_eq!(count, 4);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_likes_count_with_dislikes(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        tokio::try_join!(
            db.like_track(1, 1),
            db.like_track(2, 1),
            db.like_track(3, 1),
            db.like_track(4, 1),
            db.dislike_track(1, 1),
            db.dislike_track(5, 1),
        )?;
        let count = db.get_likes_count_for_track(1).await?;
        assert_eq!(count, 3);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_liked_tracks(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_track(1, 1).await?;
        db.like_track(1, 3).await?;
        db.like_track(1, 5).await?;
        db.like_track(1, 4).await?;
        let tracks = db.get_liked_tracks_for_user(1, 1, 10).await?;
        assert_eq!(tracks, vec![4, 5, 3, 1]);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_liked_tracks_with_pagination(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_track(1, 1).await?;
        db.like_track(1, 3).await?;
        db.like_track(1, 5).await?;
        db.like_track(1, 4).await?;
        let tracks = db.get_liked_tracks_for_user(1, 2, 2).await?;
        assert_eq!(tracks, vec![3, 1]);
        Ok(())
    }
}