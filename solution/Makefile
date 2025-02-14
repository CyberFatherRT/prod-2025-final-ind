DOCKER ?= docker

.PHONY: lint
lint:
	@echo "Running linter..."
	@cargo clippy -- -D warnings

.PHONY: test
test:
	@echo "Running tests..."
	@cargo test --test '*'

.PHONY: test-coverage
test-coverage:
	@echo "Running tests with coverage..."
	@cargo tarpaulin

.PHONY: build
build:
	@echo "Building..."
	@cargo build

.PHONY: docker-build
docker-build:
	@echo "Building docker image..."
	@${DOCKER} compose build

.PHONY: up
up:
	@echo "Starting docker containers..."
	@${DOCKER} compose up -d

.PHONY: down
down:
	@echo "Stopping docker containers..."
	@${DOCKER} compose down

.PHONY: ps
ps:
	@echo "Showing logs..."
	@${DOCKER} compose ps -a
