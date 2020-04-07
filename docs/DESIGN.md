# Design
## Componnents
This project have 3 server components, one client with CLI and tests.
1. Catalog server - It is in `src/pygmy-backend`
2. Order server - It is in `src/pygmy-backend`
3. Frontend server - It is in `src/pygmy-frontend`
4. Client and tests - It is in `tests/pygmy-client`
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
To run client against a frontend machine.

## How to deploy and run on a remote machine
You can deploy those 3 servers to any of the remote hosts if you have their SSH crendentials. This project provided a script to do so. 
After you built the server from previous steps. You can deploy by:
First, change the script file in `src/deployment/deploy.sh`, set server and user names as you needed.
Copy binaries into deployment folder
```
cp ./catalog-server src/deployment
cp ./order-server src/deployment
cp ./pygmy-frontend src/deployment
```
Note that those binary have to be built on a machine have the same operating system and architecture. For deploying to ELab, you need a Linux machine with amd64 architecture.

Next, run the deployment under its folder
```
cd src/deployment/
./deploy.sh
cd ../../
```
You need to input passwords for a cople of time. The script will use `rsync` to upload every files in the `deployment` directory to remote directory, and run server components as specified in the script.

After the deployment, you can play around with the client. Note that ELab did not opened ports for those servers, you need to run the client on ELab machine.

## How to run tests
Tests are in clients. After you build the client, you can see test as an option.
```
./pygmy-client <Frontend Server URL>
```
You will see
```
Welcome to Pygmy.com - the World’s smallest book store
Server URL is: <Frontend Server URL>
You can
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
**5. Test**
6. Leave
Select by input the number: 
```
Number 5 is tests, input 5 to test.