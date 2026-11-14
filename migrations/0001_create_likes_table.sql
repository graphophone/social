CREATE TABLE IF NOT EXISTS follows (
    followee_id BIGINT NOT NULL,
    follower_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    UNIQUE (followee_id, follower_id)
);

CREATE INDEX idx_follows_follower
ON follows (follower_id);

CREATE TABLE IF NOT EXISTS comments (
    comment_id BIGINT NOT NULL,
    commenter_id BIGINT NOT NULL,
    music_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
