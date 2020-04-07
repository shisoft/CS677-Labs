# Design
## Componnents
This project have 3 server components, one client with CLI and tests.
1. Catalog server - It is in src/pygmy-backend
2. Order server - It is in src/pygmy-backend
3. Frontend server - It is in src/pygmy-frontend
4. Client and tests - It is in tests/pygmy-client
Server address and ports, as well as endpoints for their services, can be configured via `.env` file.

## How to build
To build this project, you need to run following commands. Although Rust toolchains is cross platform, its product, which is the binary file is not. To run the program on ELab, you need a Linux machine with amd64 architecture to compile everything.
First, install the toolchain
```
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env
```
Then, build everything in their directory
```
cd src/pygmy-backend/
cargo build --release
cp target/release/catalog-server ../../
cp target/release/order-server ../../

cd ../pygmy-frontend
cargo build --release
cp target/release/pygmy-frontend ../../

cd ../../tests/pygmy-client
cargo build --release
cp target/release/pygmy-client ../../

cd ../../
```
Now you have binary in the root directory.

## How to run
To run the cluster, `catalog-server`, `order-server` both require `db` and `.env` file. You can change `.env` file for your needs. `pygmy-frontend` require `.env` file only. Client is self sustainable. 

There should be no external library required. You are free to move those files around as long as they are at the same directory.

To run the server, you can simply execute them. For example
```
./catalog-server
```
For catalog server
```
./order-server
```
For order server
```
./pygmy-frontend
```
For frontend
```
./pygmy-client <Frontend Server URL>
```
To run client against a frontend machine