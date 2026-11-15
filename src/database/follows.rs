use crate::database::SocialDb;

pub trait FollowsDb {
    async fn get_user_follows_count(&self, user_id: i64) -> sqlx::Result<(i64, i64)>;
    async fn toggle_follow(&self, followee_id: i64, follower_id: i64) -> sqlx::Result<()>;
    async fn get_followers(&self, user_id: i64, page_number: i64) -> sqlx::Result<(i64, Vec<i64>)>;
    async fn get_followings(&self, user_id: i64, page_number: i64)
    -> sqlx::Result<(i64, Vec<i64>)>;
}

impl FollowsDb for SocialDb {
    async fn get_user_follows_count(&self, user_id: i64) -> sqlx::Result<(i64, i64)> {
        let res = sqlx::query!(
            r"
            SELECT
                (SELECT COUNT(*) FROM follows WHERE follower_id = $1) followings_count,
                (SELECT COUNT(*) FROM follows WHERE followee_id = $1) followers_count
            ",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((
            res.followings_count.unwrap_or(0),
            res.followers_count.unwrap_or(0),
        ))
    }

    async fn toggle_follow(&self, followee_id: i64, follower_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            r"
            WITH ctx AS (
                DELETE FROM follows
                WHERE followee_id = $1 AND follower_id = $2
                RETURNING 1
            )
            INSERT INTO follows (followee_id, follower_id)
            SELECT $1, $2
            WHERE NOT EXISTS (SELECT 1 FROM ctx)
            ON CONFLICT (followee_id, follower_id) DO NOTHING;",
            followee_id,
            follower_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_followers(&self, user_id: i64, page_number: i64) -> sqlx::Result<(i64, Vec<i64>)> {
        let mut tx = self.pool.begin().await?;

        let user_ids = sqlx::query_scalar!(
            r"
            SELECT follower_id
            FROM follows
            WHERE followee_id = $1
            ORDER BY created_at
            LIMIT 32 OFFSET $2;",
            user_id,
            (page_number - 1) * 32
        )
        .fetch_all(&mut *tx)
        .await?;

        let count = sqlx::query_scalar!(
            r"
            SELECT COUNT(*)
            FROM follows
            WHERE followee_id = $1;",
            user_id
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        let count = (count.unwrap_or(0) as f64 / 32.0).ceil() as i64;

        Ok((count, user_ids))
    }

    async fn get_followings(
        &self,
        user_id: i64,
        page_number: i64,
    ) -> sqlx::Result<(i64, Vec<i64>)> {
        let mut tx = self.pool.begin().await?;

        let user_ids = sqlx::query_scalar!(
            r"
            SELECT followee_id
            FROM follows
            WHERE follower_id = $1
            ORDER BY created_at
            LIMIT 32 OFFSET $2;",
            user_id,
            (page_number - 1) * 32
        )
        .fetch_all(&mut *tx)
        .await?;

        let count = sqlx::query_scalar!(
            r"
            SELECT COUNT(*)
            FROM follows
            WHERE follower_id = $1;",
            user_id,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        let count = (count.unwrap_or(0) as f64 / 32.0).ceil() as i64;

        Ok((count, user_ids))
    }
}
