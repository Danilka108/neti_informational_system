INSERT
    INTO user_sessions (user_id, metadata, refresh_token, expires_at_in_seconds)
    VALUES ($1, $2, $3, $4)
    ON CONFLICT DO NOTHING
    RETURNING *;
