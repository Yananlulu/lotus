FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

# packages
RUN apt-get update && apt-get -y install apt-utils apt-transport-https wget gnupg

RUN wget http://www.rabbitmq.com/rabbitmq-signing-key-public.asc
RUN apt-key add rabbitmq-signing-key-public.asc
RUN echo "deb https://dl.bintray.com/rabbitmq/debian/ bionic main" > /etc/apt/sources.list.d/rabbitmq.list

RUN wget https://artifacts.elastic.co/GPG-KEY-elasticsearch
RUN apt-key add GPG-KEY-elasticsearch
RUN echo "deb https://artifacts.elastic.co/packages/6.x/apt stable main" > /etc/apt/sources.list.d/elasticsearch.list

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 6B73A36E6026DFCA

RUN apt-get update \
  && apt-get -y upgrade \
  && apt-get -y install build-essential pkg-config zsh net-tools psmisc inotify-tools logrotate \
  zip unzip telnet curl git vim pwgen sudo \
  libsodium-dev libpq-dev libmysqlclient-dev libsqlite3-dev\
  libssl-dev libreadline-dev zlib1g-dev \
  postgresql mysql-server redis rabbitmq-server openssh-server nginx supervisor \
  && apt-get -y autoremove \
  && apt-get -y clean

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

ADD oh-my-zsh.sh /oh-my-zsh.sh
RUN /oh-my-zsh.sh
ADD nvm.sh /nvm.sh
RUN /nvm.sh
ADD sdkman.sh /sdkman.sh
RUN /sdkman.sh
ADD rbenv.sh /rbenv.sh
RUN /rbenv.sh
ADD rustup.sh /rustup.sh
RUN /rustup.sh
ADD elasticsearch.sh /elasticsearch.sh
RUN /elasticsearch.sh

# setup
USER root
RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8

RUN /usr/sbin/rabbitmq-plugins enable rabbitmq_management
RUN chown rabbitmq:rabbitmq /var/lib/rabbitmq/.erlang.cookie
RUN chmod 600 /var/lib/rabbitmq/.erlang.cookie

RUN mkdir /var/run/sshd

RUN mkdir -p /var/run/mysqld
RUN chown -R mysql:mysql /var/run/mysqld

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

EXPOSE 3000/tcp 8080/tcp 22/tcp 3306/tcp 5432/tcp 6379/tcp 9200/tcp 5672/tcp 15672/tcp

VOLUME /app /home/deploy/.ssh

CMD ["/usr/bin/supervisord"]
