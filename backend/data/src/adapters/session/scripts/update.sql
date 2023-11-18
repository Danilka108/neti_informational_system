UPDATE
    user_sessions
    SET
        refresh_token = $3,
        expires_at_in_seconds = $4
    WHERE
        user_id = $1 AND metadata = $2
    RETURNING *;
