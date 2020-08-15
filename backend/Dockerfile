FROM rust:slim

WORKDIR /usr/src/backend
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["openedx-admin-backend"]
