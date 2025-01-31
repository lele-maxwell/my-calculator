# stage 1

FROM rust:latest as builder

WORKDIR /app/

COPY . . 

RUN cargo build --release

RUN apt-get update  

#stage 2


FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/calculator /app/calculator

RUN chmod +x /app/calculator



CMD ["./calculator"]