docker compose up -d
$ENV:DATABASE_URL = "postgres://dev:developer@localhost/trainmon"
cargo watch -x run
