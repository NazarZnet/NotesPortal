-- Your SQL goes here
CREATE TABLE IF NOT EXISTS important_posts (
    user_id UUID REFERENCES users(id),
    post_id UUID REFERENCES posts(id),
    PRIMARY KEY (user_id, post_id)
);