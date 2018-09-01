#!/bin/sh

VERSION=6.4.0

cd /tmp
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-$VERSION.tar.gz
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-$VERSION.tar.gz.sha512
shasum -a 512 -c elasticsearch-$VERSION.tar.gz.sha512
tar -xzf elasticsearch-$VERSION.tar.gz -C $HOME/elasticsearch
