# Launching locally
1. Run `docker-compose build && docker-compose --env-file .env.dev up` in the project's directory.
2. Navigate to `http://127.0.0.1:8000`.

# Generate sqlx-data.json
1. docker run -it --rm -e POSTGRES_USER=test -e POSTGRES_PASSWORD=test -p 5678:5432 postgres
2. cargo sqlx migrate run --database-url postgresql://test:test@localhost:5678
3. cargo sqlx prepare --database-url postgresql://test:test@localhost:5678