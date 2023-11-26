FROM rust:1.73-buster

COPY . /app
WORKDIR /app

COPY . /app

RUN sed -i -e 's/\r$//' /app/your_docker.sh

ENTRYPOINT [ "/app/your_docker.sh --directory /assets" ]

EXPOSE 4221
