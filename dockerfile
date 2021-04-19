# ref https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/

#FROM scratch
FROM alpine:latest

ADD backend/target/x86_64-unknown-linux-musl/release/sse-chat-backend /
COPY backend/public/ /public/
EXPOSE 5050

CMD ["/sse-chat-backend"]