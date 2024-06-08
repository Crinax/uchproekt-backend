FROM rust as builder

WORKDIR /usr/src/uchproekt-backend

COPY . .

RUN cargo build --release

FROM alpine AS RUNNER

COPY --from=builder /usr/src/uchproekt-backend/target/release/uchproekt-backend .

CMD ["./uchproekt-backend"]
