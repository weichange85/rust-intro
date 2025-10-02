FROM rust:latest as builder
ENV APP calc
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/&APP
#export this arctix web service to port 8000 and 0.0.0.0
EXPOSE 8000
CMD ["/usr/local/bin/calc"]