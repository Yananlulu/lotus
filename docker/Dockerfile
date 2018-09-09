FROM base/archlinux:latest

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm --needed zip unzip net-tools dnsutils wget vim pwgen inotify-tools  \
  base-devel zsh git cmake \
  mysql++ libpqxx libsodium \
  rabbitmqadmin \
  openssh redis percona-server postgresql rabbitmq elasticsearch nginx supervisor \
  jdk10-openjdk jre10-openjdk-headless
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

RUN /usr/sbin/rabbitmq-plugins enable rabbitmq_management
RUN systemctl enable sshd
RUN systemctl enable redis

# COPY supervisord.conf /etc/supervisord.conf

EXPOSE 3000/tcp 8080/tcp 22/tcp 3306/tcp 5432/tcp 6379/tcp 9200/tcp 5672/tcp 15672/tcp

VOLUME ["/sys/fs/cgroup" "/app" "/home/deploy/.ssh"]

CMD [ "/lib/systemd/systemd" ]
# CMD ["/usr/sbin/supervisord", "-c", "/etc/supervisord.conf"]
