
dev:
	@trap 'pkill -9 backend && pkill -9 deno' INT; \
		cargo watch -x 'run -- --source ./dev.sources.json' & \
		(cd frontend && deno task dev) & \
		wait
