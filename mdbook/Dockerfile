FROM rust:1.38.0
RUN apt update && apt install -y curl && cd /tmp && \
    curl -fsSLO https://github.com/rust-lang/mdBook/releases/download/v0.3.7/mdbook-v0.3.7-x86_64-unknown-linux-gnu.tar.gz && \
    tar xzf mdbook*.tar.gz -C /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/mdbook" ]
