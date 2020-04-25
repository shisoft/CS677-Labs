#!/bin/bash
SERVER_1="elnux1.cs.umass.edu"
SERVER_2="elnux2.cs.umass.edu"
SERVER_3="elnux3.cs.umass.edu"
DEPLOY_USER="haoshi"
REMOTE_DIR="~/"

echo "DATABASE_URL=db" > .env.t
echo "CATALOG_SERVER_LIST = '$SERVER_1,$SERVER_2,$SERVER_3'" >> .env.t
echo "ORDER_SERVER_LIST = '$SERVER_1,$SERVER_2,$SERVER_3'" >> .env.t
echo "FRONTEND_SERVER_ADDR = '$SERVER_1'" >> .env.t

echo "Deploy frontend server to $SERVER_1"
mv .env.t .env
echo "SERVER_ADDR=$SERVER_1" >> .env
echo "BOOTSTRAP_RAFT=true" >> .env
rsync -avI ./ $DEPLOY_USER@$FRONTEND_SERVER:$REMOTE_DIR

echo "Deploy catalog server to $SERVER_2"
mv .env.t .env
echo "SERVER_ADDR=$SERVER_2" >> .env
rsync -avI ./ $DEPLOY_USER@$CATALOG_SERVER:$REMOTE_DIR

echo "Deploy order server to $SERVER_3"
mv .env.t .env
echo "SERVER_ADDR=$SERVER_3" >> .env
rsync -avI ./ $DEPLOY_USER@$ORDER_SERVER:$REMOTE_DIR

echo "Running server 1 on $SERVER_1"
ssh $DEPLOY_USER@$FRONTEND_SERVER "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./pygmy-frontend && ./pygmy-frontend > frontend.log &'"
ssh $DEPLOY_USER@$SERVER_1 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./catalog-server && ./catalog-server > catalog.log &'"
ssh $DEPLOY_USER@$SERVER_1 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./order-server && ./order-server > order.log &'"

echo "Running server 2 on $SERVER_2"
ssh $DEPLOY_USER@$SERVER_2 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./catalog-server && ./catalog-server > catalog.log &'"
ssh $DEPLOY_USER@$SERVER_2 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./order-server && ./order-server > order.log &'"

echo "Running server 3 on $SERVER_3"
ssh $DEPLOY_USER@$SERVER_3 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./catalog-server && ./catalog-server > catalog.log &'"
ssh $DEPLOY_USER@$SERVER_3 "/bin/sh -c 'cd $REMOTE_DIR && chmod +x ./order-server && ./order-server > order.log &'"