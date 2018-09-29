# https://wiki.debian.org/CrossCompiling
# https://wiki.debian.org/Multiarch/HOWTO

FROM ubuntu:latest

RUN echo '2018-09-28' >> /version

ENV DEBIAN_FRONTEND noninteractive

# packages
RUN apt-get update
RUN apt-get -y install apt-utils apt-transport-https wget gnupg software-properties-common
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ bionic main multiverse universe" > /etc/apt/sources.list
RUN echo "deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ bionic-updates main multiverse universe" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ bionic main multiverse restricted universe" >> /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ bionic-updates main multiverse restricted universe" >> /etc/apt/sources.list
RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install build-essential dh-autoreconf zsh locales \
  curl git vim \
  gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf pkg-config-arm-linux-gnueabihf binutils-multiarch \
  libsodium-dev:armhf libpq-dev:armhf libmysqlclient-dev:armhf libsqlite3-dev:armhf libudev-dev:armhf liboping-dev:armhf libssl-dev:armhf
RUN apt-get -y autoremove
RUN apt-get -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN /bin/bash -c "source $HOME/.cargo/env \
  && rustup override set nightly \
  && rustup target add armv7-unknown-linux-gnueabihf"

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/bash"]
