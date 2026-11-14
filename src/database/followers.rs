use crate::database::SocialDb;

pub trait FollowersDb {
    async fn get_user_follows_count(&self, user_id: i64) -> sqlx::Result<(i64, i64)>;
    async fn toggle_follow(&self, followee_id: i64, follower_id: i64) -> sqlx::Result<()>;
}

impl FollowersDb for SocialDb {
    async fn get_user_follows_count(&self, user_id: i64) -> sqlx::Result<(i64, i64)> {
        let res = sqlx::query!(
            r"
            SELECT
                (SELECT COUNT(*) FROM followers WHERE follower_id = $1) following_count,
                (SELECT COUNT(*) FROM followers WHERE followee_id = $1) followed_count
            ",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((
            res.following_count.unwrap_or(0),
            res.followed_count.unwrap_or(0),
        ))
    }

    async fn toggle_follow(&self, followee_id: i64, follower_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            r"
            WITH ctx AS (
                DELETE FROM followers
                WHERE followee_id = $1 AND follower_id = $2
                RETURNING 1
            )
            INSERT INTO followers (followee_id, follower_id)
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
}
