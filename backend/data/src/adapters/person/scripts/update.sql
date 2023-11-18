UPDATE persons SET id = $1 WHERE id = $1 RETURNING id;
