FROM rust:1.39

WORKDIR /workspace

RUN apt-get update && apt-get install -y gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf

CMD ["rustc", "--version"]
