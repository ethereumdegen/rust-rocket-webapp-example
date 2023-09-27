CREATE TABLE user_sessions (
    id SERIAL PRIMARY KEY,

    session_token VARCHAR(255) UNIQUE NOT NULL, 
    public_address VARCHAR(255) NOT NULL, 

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);