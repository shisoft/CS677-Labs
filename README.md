# 677 Lab 3

This is the Git repo for 677 Lab 3. See https://marcoserafini.github.io/teaching/distributed-os/spring20/labs/lab3.html for a description of the lab. 

## Relation to Lab 2
Build prcess, client and functional test cases of this project is not much difference from Lab 2. This work is a fork of Lab 2.

## How to build
To build this project, you need to run following commands. Although Rust toolchains is cross platform, its product, which is the binary file is not. To run the program on ELab, you need a Linux machine other than Elab with amd64 architecture to compile everything.
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

## How to build for Docker
If you cannot compile on your machine, you can build the docker container for frontend and backend. Be sure to have docker engine up and running, run following command.
```
cd src/pygmy-backend/
sudo docker build --no-cache -t shisoft/pygmy-backend .

cd ../pygmy-frontend
sudo docker build --no-cache -t shisoft/pygmy-frontend .

cd ../../
```
Docker build does not produce any file. If you need to get the container without compiling it, you can simply pull if from my docker regestry on docker hub.
```
sudo docker pull shisoft/pygmy-backend
sudo docker pull shisoft/pygmy-frontend
```
Dockerfiles resides in `src/pygmy-backend/` and `src/pygmy-frontend`.

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

## How to run with Docker containers
This project provided a docker-compose file for you to run the cluster without building and configuring anything. Make sure your system have installed docker-compose if you want to try it.
```
cd src/
sudo docker-compose up 
cd ../
```
To clean up, run
```
cd src/
sudo docker-compose down
cd ../
```
In development process, tests are conducted in this way for effciency. 
This docker-compose file opens `34800` on local host for experiments.

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

### Setup
The script ptovided in this project will deploy 3 nodes each of order and catalog server to the 3 elab servers. Server 1 will bootstrap the raft consensus algorithum, which is potentially to be a leader in the first turn. Frontend server is on server 1 which is the entry point to the services.

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