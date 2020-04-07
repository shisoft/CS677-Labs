#!/bin/bash
FRONTEND_SERVER="elnux1.cs.umass.edu"
CATALOG_SERVER="elnux2.cs.umass.edu"
ORDER_SERVER="elnux3.cs.umass.edu"
DEPLOY_USER="chenhaohuang"
REMOTE_DIR="~/cs677/lab-2-huang-shi/src/"


echo "FRONTEND_SERVER = '$FRONTEND_SERVER:34841'" >> .env
echo "CAT_SERVER = '$CATALOG_SERVER:34842'" >> .env
echo "ORDER_SERVER = '$ORDER_SERVER:34843'" >> .env


echo "Running frontend server on $FRONTEND_SERVER"
ssh -n -f $DEPLOY_USER@$FRONTEND_SERVER "sh -c 'cd $REMOTE_DIR;kill -9 \$(lsof -t -i:34841);java -jar frontend/target/frontend-1.0-SNAPSHOT-jar-with-dependencies.jar &'"

echo "Running catalog server on $CATALOG_SERVER"
ssh -n -f $DEPLOY_USER@$CATALOG_SERVER "sh -c 'cd $REMOTE_DIR;kill -9 \$(lsof -t -i:34842);java -jar target/Catalog-1.0-SNAPSHOT-jar-with-dependencies.jar &'"

echo "Running order server on $ORDER_SERVER"
ssh -n -f $DEPLOY_USER@$ORDER_SERVER "sh -c 'cd $REMOTE_DIR;kill -9 \$(lsof -t -i:34843);java -jar orderservice/target/orderservice-1.0-SNAPSHOT-jar-with-dependencies.jar &'"