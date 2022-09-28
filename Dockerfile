FROM rust:1.64.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/x-server-stats
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/x-server-stats /usr/local/bin/x-server-stats

EXPOSE 8082

CMD ["x-server-stats"]