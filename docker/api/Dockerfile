FROM rust:1.58.1
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features mysql
COPY . .