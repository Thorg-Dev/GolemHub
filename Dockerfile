FROM rust
RUN cargo install cargo-build-deps
RUN cd /tmp && USER=root cargo new --bin PROJECT_NAME
WORKDIR /tmp/PROJECT_NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps
COPY src /tmp/PROJECT_NAME/src
COPY migrations /tmp/PROJECT_NAME/migrations
RUN cargo build
