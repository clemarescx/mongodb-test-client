FROM rust:1.29.2

WORKDIR /usr/src/rust_db_maintenance

COPY . /usr/src/rust_db_maintenance
# ENV MONGODB_HOST_URL "host.docker.internal" #<-- host.docker.internal refers to host system's localhost
# ENV MONGODB_HOST_PORT "27017"
# ENV DB_NAME "attensitoolkit"

RUN cat /etc/os-release
RUN openssl version
RUN cargo install

EXPOSE 27017

CMD ["mongodb-test-client"]