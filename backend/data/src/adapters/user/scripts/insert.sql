INSERT
    INTO users (id, email, hashed_password, role)
    VALUES ($1, $2, $3, $4)
    ON CONFLICT DO NOTHING
    RETURNING id, email, hashed_password, role as "role!: PgRole";
