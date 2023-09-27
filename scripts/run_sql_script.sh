cd ..

. ./.env

psql --host=localhost --port=5432 --username=$PG_USERNAME --password --dbname=$PG_DBNAME --file db.sql
