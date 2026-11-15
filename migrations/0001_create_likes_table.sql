CREATE TABLE IF NOT EXISTS follows (
    followee_id BIGINT NOT NULL,
    follower_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (followee_id, follower_id)
);

CREATE INDEX idx_follows_follower_created
ON follows (follower_id, created_at, followee_id);

CREATE TABLE IF NOT EXISTS comments (
    comment_id BIGINT NOT NULL,
    commenter_id BIGINT NOT NULL,
    music_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
