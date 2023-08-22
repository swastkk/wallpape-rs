FROM rust:latest AS rust-build
WORKDIR /app
COPY . .
RUN cargo build 
FROM debian:buster-slim
WORKDIR /app
COPY --from=rust-build /app/target/release/wallpape-rs .  
CMD ["./wallpape-rs"] 
