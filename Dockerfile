FROM alpine:3.19

RUN apk add --no-cache ca-certificates bash \
    && apk add --no-cache curl-impersonate --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/

WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/muavoucher-backend /usr/local/bin/muavoucher-backend
COPY get_link.sh /usr/local/bin
RUN chmod +x /usr/local/bin/*

CMD ["muavoucher-backend", "start"]