FROM rust:1-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p vault-node

FROM debian:bookworm-slim
RUN useradd -u 10001 -m vault
COPY --from=build /src/target/release/vault-node /usr/local/bin/vault-node
USER vault
EXPOSE 4000
ENTRYPOINT ["vault-node"]