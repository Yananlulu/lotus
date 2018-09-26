#!/bin/zsh

mkdir -p $HOME/downloads

cd $HOME/downloads
export BUILDROOT_VERSION="2018.08"
wget https://buildroot.org/downloads/buildroot-${BUILDROOT_VERSION}.tar.bz2
tar xf buildroot-${BUILDROOT_VERSION}.tar.bz2
cd buildroot-${BUILDROOT_VERSION}
make raspberrypi3_64_defconfig
make
