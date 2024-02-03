FROM rust:latest
WORKDIR /usr/src/backend
COPY . .
ARG DATABASE_URL
RUN rustup default nightly && rustup update
RUN cargo build --release
CMD ["cargo", "run", "--release"]