FROM rust:latest

WORKDIR /usr/src/app

RUN apt-get update && apt-get upgrade -qq && apt-get install -qq gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf

CMD ["sleep", "infinity"]