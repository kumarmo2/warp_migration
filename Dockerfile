FROM rust:1.43

WORKDIR /app

COPY . .

RUN cargo build

EXPOSE 8080

CMD ["./target/debug/warp_migration"]