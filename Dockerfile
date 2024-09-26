FROM rust:latest as builder
WORKDIR /usr/src/suiseiseki
COPY . .
RUN cargo install --path .

FROM balenalib/armv7hf-debian:latest
COPY --from=builder /usr/local/cargo/bin/suiseiseki /usr/local/bin/suiseiseki

EXPOSE 80
CMD ["suiseiseki"]
