# Docker container for MongoDB Rust query client

## Motivation

The performance of the "official" [`mongodb v0.3.10`](https://crates.io/crates/mongodb) crate is beyond what is acceptable for production use, let alone debugging.
The next best crate available is [`mongo_driver`](https://github.com/thijsc/mongo-rust-driver), a wrapper for the [libmongoc](http://mongoc.org/libmongoc/current/installing.html) library in C.

However, `mongo_driver` depends on `openssl-sys`, which is a pain to handle as it requires OpenSSL to be installed and neatly configured for linking.

This project is born out of my failure to bend Windows to my will and configure OpenSSL for linking with the library.

## Requirements

Developed with Docker Community Edition v18.06.1

## Build

```bash
git clone git@github.com:clemarescx/mongodb-test-client.git
cd mongodb-test-client
docker build -t mongo-c-rust .
```

## Run

Two alternatives to configure the client:

### Config from RON file:

The configuration can be placed in a `config.ron` file located in the root folder (alongside the top-most `Cargo.toml`), with the following content:

```JSON
Config (
    url: "127.0.0.1",
    port: "27017",
    db_name: "yourdatabase"
)
```

Enable the config by setting the `USE_RON_CONFIG` flag when starting the container:

`docker run -it --rm -e USE_RON_CONFIG --name running-mongo-c mongo-c-rust`

### Config from environment variables:

Send all attributes as environment variables when starting the container: 

`docker run -it --rm -e MONGODB_HOST_URL=<URL> MONGODB_HOST_PORT=<PORT> DB_NAME=<NAME> --name running-mongo-c mongo-c-rust`

E.g: _attensitoolkit_ database on host URL _localhost:27017_:

`docker run -it --rm -e MONGODB_HOST_URL=host.docker.internal -e MONGODB_HOST_PORT=27017 -e DB_NAME=attensitoolkit --name running-mongo-c mongo-c-rust`

Note: as per official [documentation](https://docs.docker.com/docker-for-windows/networking/#use-cases-and-workarounds),  `host.docker.internal` is a reference within the container to `127.0.0.1` on the host OS.
Those are the default values in this project if none are provided. So the command above could be written as:
`docker run -it --rm --name running-mongo-c mongo-c-rust`

## Cleanup

To remove the Docker container created, run:

`docker container prune`

To remove the Docker image created during the build phase, run:

1. List all created images with

>`docker images -a`

and get the id of the image named `mongo-c-rust`.

2. cleanup with:

> `docker image prune && docker rmi <image-id>`
(entering the first 4 digits of the id should suffice)