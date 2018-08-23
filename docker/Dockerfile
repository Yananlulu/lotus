FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

# packages
RUN apt-get update && apt-get -y install wget logrotate gnupg apt-utils
RUN wget http://www.rabbitmq.com/rabbitmq-signing-key-public.asc
RUN apt-key add rabbitmq-signing-key-public.asc
RUN echo "deb https://dl.bintray.com/rabbitmq/debian/ bionic main" > /etc/apt/sources.list.d/rabbitmq.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 6B73A36E6026DFCA
RUN apt-get update \
  && apt-get -y upgrade \
  && apt-get -y install curl build-essential git pkg-config zsh vim net-tools psmisc inotify-tools pwgen \
  && apt-get -y install libsodium-dev libpq-dev libmysqlclient-dev \
  && apt-get -y install libssl-dev libreadline-dev zlib1g-dev \
  && apt-get -y install postgresql redis rabbitmq-server openssh-server nginx supervisor \
  && apt-get -y autoremove \
  && apt-get -y clean

# rabbitmq
RUN /usr/sbin/rabbitmq-plugins enable rabbitmq_management

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
RUN echo "local   all             all                                     trust" > "/etc/postgresql/10/main/pg_hba.conf"
RUN echo "host    all             all             127.0.0.1/32            trust" >> "/etc/postgresql/10/main/pg_hba.conf"
RUN echo "bind 127.0.0.1" >> /etc/redis/redis.conf
RUN chown rabbitmq:rabbitmq /var/lib/rabbitmq/.erlang.cookie
RUN chmod 600 /var/lib/rabbitmq/.erlang.cookie
RUN mkdir /var/run/sshd
RUN echo 'root:toor' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords yes/' /etc/ssh/sshd_config
RUN chsh -s /bin/zsh

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

EXPOSE 3000/tcp 8080/tcp 22/tcp 5432/tcp 6379/tcp 5672/tcp 15672/tcp

VOLUME /app

CMD ["/usr/bin/supervisord"]
