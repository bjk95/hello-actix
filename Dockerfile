FROM rust:1.43.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/hello-actix /usr/local/bin/hello-actix
COPY --from=build /usr/config/ /usr/config

WORKDIR /usr/

EXPOSE 8080
CMD ["hello-actix"]