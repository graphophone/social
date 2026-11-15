CREATE TABLE IF NOT EXISTS follows (
    followee_id BIGINT NOT NULL,
    follower_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (followee_id, follower_id)
);

CREATE INDEX idx_follows_follower_created
ON follows (follower_id, created_at, followee_id);

CREATE TABLE IF NOT EXISTS comments (
    id BIGSERIAL PRIMARY KEY,
    commenter_id BIGINT NOT NULL,
    music_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS comment_likes (
    comment_id BIGINT REFERENCES comments (id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL,
    UNIQUE (user_id, comment_id)
);

CREATE INDEX idx_comments_music_id
ON comments (music_id, created_at);
