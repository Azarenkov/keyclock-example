FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef
WORKDIR /keyclock-example

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /keyclock-example/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /keyclock-example/target/x86_64-unknown-linux-musl/release/keyclock-example /keyclock-example
ENTRYPOINT ["/keyclock-example"]
EXPOSE 8081
