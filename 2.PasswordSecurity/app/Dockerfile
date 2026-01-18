FROM rust:slim AS builder
ARG USERNAMES_FILE

WORKDIR /app
ENV RUSTFLAGS="-C target-cpu=generic"
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY ${USERNAMES_FILE} ./usernames.csv
RUN cargo build --release --locked

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/timing-attack-demo /usr/local/bin/timing-attack-demo
COPY --from=builder /app/usernames.csv /app/usernames.csv
COPY --from=builder /app/templates ./templates
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/timing-attack-demo", "--port", "3000", "-c", "/app/usernames.csv"]
