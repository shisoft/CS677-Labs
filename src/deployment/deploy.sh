#!/bin/bash
FRONTEND_SERVER="elnux1.cs.umass.edu"
CATALOG_SERVER="elnux2.cs.umass.edu"
ORDER_SERVER="elnux3.cs.umass.edu"
DEPLOY_USER="haoshi"
REMOTE_DIR="~/"

echo "file:db" > .env
echo "CAT_SERVER_ADDR = '$CATALOG_SERVER:34801'" >> .env
echo "ORDER_SERVER_ADDR = '$ORDER_SERVER:34802'" >> .env

echo "Deploy frontend server to $FRONTEND_SERVER"
rsync -av ./ $DEPLOY_USER@$FRONTEND_SERVER:$REMOTE_DIR

echo "Deploy catalog server to $CATALOG_SERVER"
rsync -av ./ $DEPLOY_USER@$CATALOG_SERVER:$REMOTE_DIR

echo "Deploy order server to $ORDER_SERVER"
rsync -av ./ $DEPLOY_USER@$ORDER_SERVER:$REMOTE_DIR

echo "Running frontend server on $FRONTEND_SERVER"
ssh -n -f $DEPLOY_USER@$FRONTEND_SERVER "sh -c 'cd $REMOTE_DIR;chmod +x ./pygmy-frontend; nohup ./pygmy-frontend > frontend.log &'"

echo "Running catalog server on $CATALOG_SERVER"
ssh -n -f $DEPLOY_USER@$CATALOG_SERVER "sh -c 'cd $REMOTE_DIR;chmod +x ./catalog-server; nohup ./catalog-server > catalog-server.log &'"

echo "Running order server on $ORDER_SERVER"
ssh -n -f $DEPLOY_USER@$ORDER_SERVER "sh -c 'cd $REMOTE_DIR;chmod +x ./order-server; nohup ./order-server > order-server.log &'"