FROM rust:1.73-alpine

COPY . /app
WORKDIR /app

RUN sed -i -e 's/\r$//' /app/your_server.sh

ENTRYPOINT [ "/app/your_server.sh ]

EXPOSE 4221
