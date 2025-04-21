build-database:
	podman pull postgres:latest
	podman run --name rust-quest-tracker-db -e POSTGRES_PASSWORD=123456 -e POSTGRES_DB=rustdb -p 5432:5432 -d postgres:latest

migrate-up:
	diesel migration run

migrate-down:
	diesel migration revert

build-app:
	podman build -t rust-quest-tracker:v1.0.0 .

# Edit your .env file to set the correct database URL and JWT secrets before running this command.
start-container:
	podman run --name rust-quest-tracker -p 8080:8080 \
		-e STAGE=Local \
		-e SERVER_PORT=8080 \
		-e SERVER_BODY_LIMIT=10 \
		-e SERVER_TIMEOUT=90 \
		-e DATABASE_URL=postgres://postgres:12345@localhost:5432/rustdb \
		-e JWT_USER_SECRET=xxxx \
		-e JWT_USER_REFRESH_SECRET=xxxx \
		-e JWT_ADMIN_SECRET=xxxx \
		-e JWT_ADMIN_REFRESH_SECRET=xxxx \
		-d rust-quest-tracker:v1.0.0

tests:
	cargo tarpaulin --out html