INSERT INTO test(value, ts_created)
VALUES ($1, now())
RETURNING id;