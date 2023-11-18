UPDATE users
    SET
        email = $1,
        hashed_password = $2,
        role = $3
    WHERE id = $4
    RETURNING id, email, hashed_password, role as "role!: PgRole";
