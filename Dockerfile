# Multi-stage Dockerfile for Go application
FROM golang:1.26.1 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssh-client git

RUN mkdir -p -m 0700 ~/.ssh && \
    ssh-keyscan git.kundeng.us >> ~/.ssh/known_hosts

# Configure Git to use SSH for GitHub
RUN git config --global url."ssh://git@git.kundeng.us".insteadOf "https://git.kundeng.us"

# Set up the Go environment for private modules
ENV GOPRIVATE=git.kundeng.us


# Copy go mod and sum files
COPY go.mod go.sum ./

RUN --mount=type=ssh mkdir src && \
    go mod download

# Copy source code
COPY ./cmd ./cmd
COPY ./internal ./internal
COPY ./Makefile .
COPY ./.env .
COPY ./migrations ./migrations
COPY ./docs ./docs

# Build the application
RUN CGO_ENABLED=0 GOOS=linux make build

# Runtime stage
FROM alpine:latest AS production

RUN apk --no-cache add ca-certificates

WORKDIR /root/

# Copy the pre-built binary file from the previous stage
COPY --from=builder /app/textsender-auth .
COPY --from=builder /app/.env ./
COPY --from=builder /app/migrations ./migrations

# Expose port
EXPOSE 9080

# Command to run the executable
CMD ["./textsender-auth"]
