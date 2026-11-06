FROM rust:1.89-bookworm AS build
WORKDIR /src
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /src/target/release/ai-multi-os-lab-api /usr/local/bin/lab-api
USER 65532:65532
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/lab-api"]
