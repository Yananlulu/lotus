FROM base/archlinux:latest

RUN echo '2018-09-09' >> /version

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm --needed zip unzip net-tools dnsutils wget vim pwgen inotify-tools  man\
  base-devel zsh git cmake libsodium \
  percona-server percona-server-clients mysql++ \
  postgresql libpqxx \
  rabbitmq rabbitmqadmin \
  openssh redis nginx supervisor \
  elasticsearch jdk10-openjdk jre10-openjdk-headless
RUN pacman -Scc --noconfirm

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

# setup
USER root
RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN echo "LANG=en_US.UTF-8" > /etc/locale.conf

RUN ssh-keygen -A

RUN su - postgres -c "initdb --locale en_US.UTF-8 -D '/var/lib/postgres/data'"
RUN mkdir -p /run/postgresql
RUN chown -R postgres:postgres /run/postgresql

RUN mysqld --initialize-insecure --user=mysql

RUN rabbitmq-plugins enable rabbitmq_management
RUN chown rabbitmq:rabbitmq /var/lib/rabbitmq/.erlang.cookie
RUN chmod 600 /var/lib/rabbitmq/.erlang.cookie
RUN sed -i '/^NODENAME/ d' /etc/rabbitmq/rabbitmq-env.conf

RUN chown -R elasticsearch:elasticsearch /etc/elasticsearch

COPY supervisord.conf /etc/supervisord.conf

EXPOSE 3000/tcp 8080/tcp 22/tcp 3306/tcp 5432/tcp 6379/tcp 9200/tcp 5672/tcp 15672/tcp

VOLUME ["/workspace", "/home/deploy/.ssh"]

CMD ["/usr/sbin/supervisord", "-c", "/etc/supervisord.conf"]
