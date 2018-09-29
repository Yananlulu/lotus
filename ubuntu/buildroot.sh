#!/bin/zsh

mkdir -p $HOME/downloads

cd $HOME/downloads
export BUILDROOT_VERSION="2018.08"
export BUILDROOT_HOME=$HOME/local/buildroot
mkdir -p $BUILDROOT_HOME
wget https://buildroot.org/downloads/buildroot-$BUILDROOT_VERSION.tar.bz2
tar xf buildroot-$BUILDROOT_VERSION.tar.bz2 -C $BUILDROOT_HOME
