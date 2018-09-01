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
  && apt-get -y install curl build-essential git pkg-config zsh vim net-tools psmisc inotify-tools pwgen sudo logrotate \
  && apt-get -y install libsodium-dev libpq-dev libmysqlclient-dev libsqlite-dev\
  && apt-get -y install libssl-dev libreadline-dev zlib1g-dev \
  && apt-get -y install openjdk-11-jdk \
  && apt-get -y install postgresql mysql-server redis rabbitmq-server elasticsearch openssh-server nginx supervisor \
  && apt-get -y autoremove \
  && apt-get -y clean

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN echo 'deploy:123456' | chpasswd
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

# zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true

# rbenv
RUN git clone https://github.com/sstephenson/rbenv.git $HOME/.rbenv
RUN git clone https://github.com/sstephenson/ruby-build.git $HOME/.rbenv/plugins/ruby-build
RUN git clone https://github.com/rbenv/rbenv-vars.git $HOME/.rbenv/plugins/rbenv-vars
RUN echo 'export PATH=$HOME/.rbenv/bin:$PATH' >> $HOME/.zshrc
RUN echo 'eval "$(rbenv init -)"' >> $HOME/.zshrc


# rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.zshrc

# nvm
RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | zsh

COPY install.sh /install.sh
RUN /install.sh



# setup
USER postgres
RUN /etc/init.d/postgresql start \
  && psql --command "CREATE USER www WITH SUPERUSER PASSWORD '123456';"

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

EXPOSE 3000/tcp 8080/tcp 22/tcp

VOLUME /app

CMD ["/usr/bin/supervisord"]
