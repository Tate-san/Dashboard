FROM docker.io/rust:1.74-alpine as build

RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev


WORKDIR /usr/src/dashboard_server
COPY . .
RUN cargo install --path . 

FROM docker.io/alpine:latest 
ARG ARCH=aarch64
COPY --from=build /usr/local/cargo/bin/dashboard_server /usr/local/bin/dashboard_server

CMD ["dashboard_server"]