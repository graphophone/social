use serde::Deserialize;

use crate::database::SocialDb;

pub trait FollowsDb {
    async fn get_follows_count(
        &self,
        user_id: i64,
        follower_id: Option<i64>,
    ) -> sqlx::Result<FollowsCount>;
    async fn toggle_follow(&self, followee_id: i64, follower_id: i64) -> sqlx::Result<()>;
    async fn get_followers(
        &self,
        user_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<FollowEntry>)>;
    async fn get_followings(
        &self,
        user_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<FollowEntry>)>;
}

impl FollowsDb for SocialDb {
    async fn get_follows_count(
        &self,
        user_id: i64,
        follower_id: Option<i64>,
    ) -> sqlx::Result<FollowsCount> {
        let res = sqlx::query!(
            r"
            SELECT
                (SELECT COUNT(*) FROM follows WHERE follower_id = $1) followings_count,
                (SELECT COUNT(*) FROM follows WHERE followee_id = $1) followers_count,
                (SELECT EXISTS (
                    SELECT 1 FROM follows
                    WHERE follower_id = $2 AND followee_id = $1)
                ) is_followed
            ",
            user_id,
            follower_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(FollowsCount {
            followings_count: res.followings_count.unwrap_or(0),
            followers_count: res.followers_count.unwrap_or(0),
            is_followed: res.is_followed.unwrap_or(false),
        })
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

    async fn get_followers(
        &self,
        user_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<FollowEntry>)> {
        let mut tx = self.pool.begin().await?;

        let follows = sqlx::query!(
            r"
            SELECT F1.follower_id,
                (SELECT COUNT(*) FROM follows WHERE follower_id = F1.follower_id) followings_count,
                (SELECT COUNT(*) FROM follows WHERE followee_id = F1.follower_id) followers_count,
                (SELECT EXISTS (
                    SELECT 1 FROM follows
                    WHERE follower_id = $3 AND followee_id = F1.follower_id)
                ) is_followed
            FROM follows F1
            WHERE F1.followee_id = $1
            ORDER BY F1.created_at
            LIMIT 32 OFFSET $2;",
            user_id,
            (page_number - 1) * 32,
            requester_id,
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|v| FollowEntry {
            user_id: v.follower_id,
            follows_count: FollowsCount {
                followings_count: v.followings_count.unwrap_or(0),
                followers_count: v.followers_count.unwrap_or(0),
                is_followed: v.is_followed.unwrap_or(false),
            },
        })
        .collect();

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

        Ok((count, follows))
    }

    async fn get_followings(
        &self,
        user_id: i64,
        page_number: i64,
        requester_id: Option<i64>,
    ) -> sqlx::Result<(i64, Vec<FollowEntry>)> {
        let mut tx = self.pool.begin().await?;

        let follows = sqlx::query!(
            r"
            SELECT F1.followee_id,
                (SELECT COUNT(*) FROM follows WHERE follower_id = F1.followee_id) followings_count,
                (SELECT COUNT(*) FROM follows WHERE followee_id = F1.followee_id) followers_count,
                (SELECT EXISTS (
                    SELECT 1 FROM follows
                    WHERE follower_id = $3 AND followee_id = F1.followee_id)
                ) is_followed
            FROM follows F1
            WHERE F1.follower_id = $1
            ORDER BY F1.created_at
            LIMIT 32 OFFSET $2;",
            user_id,
            (page_number - 1) * 32,
            requester_id,
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|v| FollowEntry {
            user_id: v.followee_id,
            follows_count: FollowsCount {
                followings_count: v.followings_count.unwrap_or(0),
                followers_count: v.followers_count.unwrap_or(0),
                is_followed: v.is_followed.unwrap_or(false),
            },
        })
        .collect();

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

        Ok((count, follows))
    }
}

#[derive(Deserialize)]
pub struct FollowsCount {
    pub followings_count: i64,
    pub followers_count: i64,
    pub is_followed: bool,
}

#[derive(Deserialize)]
pub struct FollowEntry {
    pub user_id: i64,
    pub follows_count: FollowsCount,
}
