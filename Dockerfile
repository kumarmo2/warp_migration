FROM rust:1.43

WORKDIR /app

COPY . .

ENV DATABASE_URL=mysql://root:root@127.0.0.1/practice

RUN cargo build

EXPOSE 8080

CMD ["./target/debug/warp_migration"]