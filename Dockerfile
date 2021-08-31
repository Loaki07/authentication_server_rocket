FROM rust:1.31

WORKDIR /authentication_server_rocker
COPY . /authentication_server_rocker/

RUN cargo run 
EXPOSE 7001