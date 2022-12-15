FROM rust

COPY . /bin/

EXPOSE 8080

CMD ["/bin/target/debug/actix-server-side-program"]
