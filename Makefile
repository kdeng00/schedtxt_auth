VERSION ?= $(shell git describe --tags 2>/dev/null || echo "dev")
COMMIT ?= $(shell git rev-parse --short HEAD)
BUILD_TIME ?= $(shell date -u +%Y-%m-%dT%H:%M:%SZ)
GO_VERSION ?= $(shell go version | awk '{print $$3}')

.PHONY: build
build:
	go build -ldflags="\
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.Version=$(VERSION)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.BuildTime=$(BUILD_TIME)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.Commit=$(COMMIT)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.GoVersion=$(GO_VERSION)'" \
		-o textsender-auth cmd/api/main.go

.PHONY: install
install:
	go install -ldflags="\
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.Version=$(VERSION)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.BuildTime=$(BUILD_TIME)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.Commit=$(COMMIT)' \
		-X 'git.kundeng.us/phoenix/textsender-auth/internal/version.GoVersion=$(GO_VERSION)'"
