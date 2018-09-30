FROM ubuntu:latest

RUN echo '2018-09-28' >> /version

ENV DEBIAN_FRONTEND noninteractive

# packages
RUN apt-get update
RUN apt-get -y install apt-utils apt-transport-https wget gnupg software-properties-common
RUN add-apt-repository -y ppa:webupd8team/java
RUN echo "oracle-java8-installer shared/accepted-oracle-license-v1-1 select true" | debconf-set-selections

RUN wget http://www.rabbitmq.com/rabbitmq-signing-key-public.asc
RUN apt-key add rabbitmq-signing-key-public.asc
RUN echo "deb https://dl.bintray.com/rabbitmq/debian bionic release" > /etc/apt/sources.list.d/rabbitmq.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 6B73A36E6026DFCA

RUN wget https://artifacts.elastic.co/GPG-KEY-elasticsearch
RUN apt-key add GPG-KEY-elasticsearch
RUN echo "deb https://artifacts.elastic.co/packages/6.x/apt stable main" > /etc/apt/sources.list.d/elasticsearch.list

RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install build-essential clang cmake bc pkg-config dh-autoreconf cpio meson intltool libtool gawk texinfo bison zsh \
  libncurses5-dev \
  net-tools lsof iputils-ping dnsutils psmisc inotify-tools logrotate \
  zip unzip bsdtar telnet curl git vim pwgen sudo gperf \
  libsodium-dev libpq-dev libmysqlclient-dev libsqlite3-dev libudev-dev liboping-dev \
  libssl-dev libreadline-dev zlib1g-dev oracle-java8-installer \
  sqlite3 postgresql mariadb-server redis rabbitmq-server openssh-server nginx supervisor
RUN apt-get -y install elasticsearch
RUN apt-get -y autoremove
RUN apt-get -y clean

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

ADD oh-my-zsh.sh /oh-my-zsh.sh
RUN /oh-my-zsh.sh
ADD android.sh /android.sh
RUN /android.sh
ADD buildroot.sh /buildroot.sh
RUN /buildroot.sh
ADD nvm.sh /nvm.sh
RUN /nvm.sh
ADD sdkman.sh /sdkman.sh
RUN /sdkman.sh
ADD rbenv.sh /rbenv.sh
RUN /rbenv.sh
ADD rustup.sh /rustup.sh
RUN /rustup.sh

# setup
USER root

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN /usr/sbin/rabbitmq-plugins enable rabbitmq_management
RUN chown rabbitmq:rabbitmq /var/lib/rabbitmq/.erlang.cookie
RUN chmod 600 /var/lib/rabbitmq/.erlang.cookie

RUN mkdir /var/run/sshd

RUN mkdir -p /var/run/mysqld
RUN chown -R mysql:mysql /var/run/mysqld

RUN mkdir -p /var/run/postgresql/10-main.pg_stat_tmp
RUN chown -R postgres:postgres /var/run/postgresql
RUN echo "local   all             all                                     trust" > /etc/postgresql/10/main/pg_hba.conf
RUN echo "host    all             all             127.0.0.1/32            trust" >> /etc/postgresql/10/main/pg_hba.conf
RUN echo "host    all             all             ::1/128                 trust" >> /etc/postgresql/10/main/pg_hba.conf

COPY supervisord.conf /etc/supervisord.conf

EXPOSE 3000/tcp 8080/tcp 22/tcp 3306/tcp 5432/tcp 6379/tcp 9200/tcp 5672/tcp 15672/tcp 10000-11000/tcp

VOLUME /workspace /home/deploy/.ssh

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
