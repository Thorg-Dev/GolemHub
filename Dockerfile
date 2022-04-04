FROM rust
RUN cargo install cargo-build-deps
RUN cd /tmp && USER=root cargo new --bin PROJECT_NAME
WORKDIR /tmp/PROJECT_NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps
COPY src /tmp/PROJECT_NAME/src
RUN cargo build