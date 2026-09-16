use crate::database;

pub trait LikesDb {
    async fn like_post(&self, user_id: i64, post_id: i64) -> Result<(), sqlx::Error>;
    async fn dislike_post(&self, user_id: i64, post_id: i64) -> Result<(), sqlx::Error>;
    async fn get_liked_posts_for_user(&self, user_id: i64, page_number: i32, page_size: i32) -> Result<Vec<i64>, sqlx::Error>;
    async fn get_likes_count_for_post(&self, post_id: i64) -> Result<i64, sqlx::Error>;
    async fn get_liked_posts_count_for_user(&self, user_id: i64) -> Result<i64, sqlx::Error>;
}

impl LikesDb for database::SocialDb {
    async fn like_post(&self, user_id: i64, post_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO post_likes (user_id, post_id) VALUES ($1, $2);", user_id, post_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn dislike_post(&self, user_id: i64, post_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r"
            DELETE FROM post_likes
            WHERE user_id = $1 AND post_id = $2;
        ", user_id, post_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_liked_posts_for_user(&self, user_id: i64, page_number: i32, page_size: i32) -> Result<Vec<i64>, sqlx::Error> {
        let offset: i32 = (page_number - 1) * page_size;
        let posts_ids = sqlx::query_scalar!(r"
            SELECT post_id FROM post_likes
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        ", user_id, page_size as i64, offset as i64)
            .fetch_all(&self.pool)
            .await?;
        Ok(posts_ids)
    }

    async fn get_liked_posts_count_for_user(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(r"
            SELECT COUNT(*)
            FROM post_likes
            WHERE user_id = $1;
        ", user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(count.unwrap_or_default())
    }

    async fn get_likes_count_for_post(&self, post_id: i64) -> Result<i64, sqlx::Error> {
        let count: Option<i64> = sqlx::query_scalar!(r"
            SELECT COUNT(user_id)
            FROM post_likes
            WHERE post_id = $1;
        ", post_id)
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
    async fn test_like_post(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_post(1, 1)
            .await?;
        Ok(())
    }

    #[sqlx::test]
    async fn test_dislike_post(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.dislike_post(1, 1)
            .await?;
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_likes_count_all_likes(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        tokio::try_join!(
            db.like_post(1, 1),
            db.like_post(2, 1),
            db.like_post(3, 1),
            db.like_post(4, 1),
        )?;
        let count = db.get_likes_count_for_post(1).await?;
        assert_eq!(count, 4);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_likes_count_with_dislikes(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        tokio::try_join!(
            db.like_post(1, 1),
            db.like_post(2, 1),
            db.like_post(3, 1),
            db.like_post(4, 1),
            db.dislike_post(1, 1),
            db.dislike_post(5, 1),
        )?;
        let count = db.get_likes_count_for_post(1).await?;
        assert_eq!(count, 3);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_liked_posts(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_post(1, 1).await?;
        db.like_post(1, 3).await?;
        db.like_post(1, 5).await?;
        db.like_post(1, 4).await?;
        let posts = db.get_liked_posts_for_user(1, 1, 10).await?;
        assert_eq!(posts, vec![4, 5, 3, 1]);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_liked_posts_with_pagination(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let db = SocialDb {
            pool,
        };
        db.like_post(1, 1).await?;
        db.like_post(1, 3).await?;
        db.like_post(1, 5).await?;
        db.like_post(1, 4).await?;
        let posts = db.get_liked_posts_for_user(1, 2, 2).await?;
        assert_eq!(posts, vec![3, 1]);
        Ok(())
    }
}