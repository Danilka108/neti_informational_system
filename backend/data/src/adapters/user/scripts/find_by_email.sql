SELECT id, email, hashed_password, role as "role!: PgRole"
    FROM users
    WHERE users.email = $1
