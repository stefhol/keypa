FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/api /usr/local/app/
WORKDIR /usr/local/app/
ENV PORT=8080


RUN ["chmod", "+x", "./api"]
CMD ["./api"]