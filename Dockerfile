FROM lukemathwalker/cargo-chef:latest-rust-1.70.0-alpine3.18 as chef
WORKDIR /backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /backend/target/release/backend /
CMD ["./backend"]