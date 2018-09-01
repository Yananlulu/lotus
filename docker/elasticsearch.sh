#!/bin/zsh

VERSION=6.4.0

mkdir $HOME/downloads
cd $HOME/downloads
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-$VERSION.tar.gz
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-$VERSION.tar.gz.sha512
shasum -a 512 -c elasticsearch-$VERSION.tar.gz.sha512
mkdir -p $HOME/local
tar xf elasticsearch-$VERSION.tar.gz -C $HOME/local
