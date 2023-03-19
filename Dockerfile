FROM rust:1.63.0 as builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

from debian:bullseye-slim as runtime 
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && apt-get rm-rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zerozero zerozero
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zerozero"]
