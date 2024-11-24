
dev:
	@trap 'pkill -9 backend && pkill -9 deno' INT; \
		cargo watch --ignore 'frontend/*' -x 'run -- --source ./sources.json' & \
		(cd frontend && deno task dev) & \
		wait

build:
	cargo sqlx prepare;
	docker-compose build
