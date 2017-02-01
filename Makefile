build-client:
	cd cache-client && cargo build

build-server:
	cd cache-server && cargo build

build-codec:
	cd cache-codec && cargo build

build-cache:
	cd lru-cache && cargo build

build: build-client build-server build-codec build-cache

test-client:
	cd cache-client && cargo test

test-server:
	cd cache-server && cargo test

test-codec:
	cd cache-codec && cargo test

test-cache:
	cd lru-cache && cargo test

test: test-client test-server test-codec test-cache
