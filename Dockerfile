FROM rust:1.85-slim AS build-env
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build-env /usr/src/app/target/release/albhed-translator-service /app/
EXPOSE 8080
USER nonroot:nonroot
CMD ["/app/albhed-translator-service"]