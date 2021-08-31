FROM rust:1.54

WORKDIR /authentication_server_rocker
COPY . /authentication_server_rocker/

RUN cargo build 
EXPOSE 7001
CMD cargo run
