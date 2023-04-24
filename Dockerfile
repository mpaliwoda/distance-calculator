FROM public.ecr.aws/docker/library/rust:1.68 AS builder

WORKDIR /usr/src/

RUN cargo new app

WORKDIR /usr/src/app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && rm -rf ./src/*.rs ./target/release/distance-calculator* ./target/release/.fingerprint/distance-calculator*

COPY . .

RUN cargo build --release

FROM public.ecr.aws/debian/debian:stable-slim

RUN groupadd -r --gid 10001 developers && \
    useradd -r -g developers -u 20008 distance_calculator && \
    chown -R distance_calculator:developers /usr/local/bin

COPY --from=builder /usr/src/app/target/release/distance-calculator /usr/local/bin/distance-calculator
COPY ./global_airports_database.sqlite /usr/local/bin/global_airports_database.sqlite

USER distance_calculator
WORKDIR /usr/local/bin

EXPOSE 8000

CMD ["distance-calculator"]
