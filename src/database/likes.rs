pub trait LikesDb {
    async fn like_track(&self, track_id: i64, user_id: i64) -> anyhow::Result<()>;
    async fn dislike_track(&self, track_id: i64, user_id: i64) -> anyhow::Result<()>;
    async fn get_liked_tracks_for_user(&self, user_id: i64) -> anyhow::Result<Vec<i64>>;
    async fn get_likes_count_for_track(&self, track_if: i64) -> anyhow::Result<i64>;
}