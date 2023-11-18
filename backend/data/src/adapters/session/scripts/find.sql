SELECT
    user_id, metadata, refresh_token, expires_at_in_seconds
    FROM user_sessions
    WHERE user_id = $1 and metadata = $2;
