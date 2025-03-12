# Hyperlane Cosmos Rust

This package provides proto and gRPC definitions to interact with the [Hyperlane Cosmos Module](http://github.com/bcp-innovations/hyperlane-cosmos).
This repository builds upon of [cosmrs](https://github.com/cosmos/cosmos-rust), using their workflow for Hyperlane-specific proto builds.

## Building Proto files from source

There is a docker container that automatically builds all the files from source. To re-export the generated files adjust [lib.rs](./src/lib.rs). To start the docker container simply run:

    cd gen
    docker compose up
