
CREATE TABLE IF NOT EXISTS posts(
    id UUID UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    important boolean NOT NULL DEFAULT False,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (user_id) REFERENCES users(id)
);