FROM rust AS build
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
COPY src ./src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/moved /bin/

CMD ["moved", "--location", "https://www.google.com"]



