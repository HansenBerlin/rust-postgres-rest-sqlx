FROM rust:1.60.0-bullseye AS build
WORKDIR /app
COPY . .

# RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
# RUN sqlx migrate run
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV SQLX_OFFLINE=true
RUN cargo build --release
RUN mkdir -p /app/lib
RUN cp -LR $(ldd ./target/release/rust-postgres-crud-sqlx | grep "=>" | cut -d ' ' -f 3) /app/lib

FROM scratch AS app
WORKDIR /app
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
COPY --from=build /app/lib /app/lib
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=build /app/target/release/rust-postgres-crud-sqlx rust-postgres-crud-sqlx
ENV LD_LIBRARY_PATH=/app/lib
ENV SQLX_OFFLINE=true
ENTRYPOINT ["./rust-postgres-crud-sqlx"]