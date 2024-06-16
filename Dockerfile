FROM rust:latest AS builder

WORKDIR /app

# Копируем файлы и компилируем приложение
COPY . .

RUN cargo build --release

# Финальный образ
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/myapp .

CMD ["./myapp"]
