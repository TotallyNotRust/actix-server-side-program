FROM rust:latest

COPY . .

EXPOSE 8080

RUN cargo build

CMD ["./target/debug/actix-server-side-program"]
