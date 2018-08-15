# lotus

A complete open source e-commerce solution built with Rust and React(STILL IN DEVELOPMENT).

## Usage

-   Install rust

```bash
curl https://sh.rustup.rs -sSf | sh
rustup component add rustfmt-preview
rustup component add rls-preview rust-analysis rust-src
```

-   Add to ~/.zshrc

```bash
source $HOME/.cargo/env
```

-   Install nvm

```bash
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | zsh
```

-   Close and reopen your terminal to start using

-   Install nodejs

```bash
nvm install node
```

-   Build

```bash
git clone https://github.com/saturn-xiv/lotus.git
cd lotus
make # dist.tar.xz
```

## Atom plugins

-   autosave: enable
-   file-icons
-   atom-beautify: enable beautify on save
-   ide-rust: disable toolchain update checking
-   language-babel

## Notes

-   Generate a random key

```bash
openssl rand -base64 32
```

-   ~/.npmrc

```bash
npm config set prefix '~/.npm-global'
```

-   'Peer authentication failed for user', open file "/etc/postgresql/9.5/main/pg_hba.conf" change line:

```bash
local   all             all                                     peer
TO:
local   all             all                                     md5
```

-   forgot mysql root password

create file  /tmp/reset.mysqld

```sql
SET PASSWORD FOR root@localhost = PASSWORD('change-me');
```

edit file /etc/mysql/my.cnf

```text
[mysqld]
init-file=/tmp/reset.mysqld
```

-   RabbitMQ

```bash
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl change_password guest change-me
rabbitmqctl add_user who-am-i change-me
rabbitmqctl set_user_tags who-am-i administrator
rabbitmqctl list_vhosts
rabbitmqctl add_vhost v-host
rabbitmqctl set_permissions -p v-host who-am-i ".*" ".*" ".*"
```

## Documents

-   [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
-   [favicon.ico](http://icoconvert.com/)
-   [smver](http://semver.org/)
-   [banner.txt](http://patorjk.com/software/taag/)
-   [msmtp](https://wiki.archlinux.org/index.php/msmtp)
-   [Are we (I)DE yet?](https://areweideyet.com/)
-   [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
-   [GraphQL](https://graphql.org/learn/)
-   [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
-   [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
-   [Let’s Encrypt](https://letsencrypt.org/)
-   [Certbot](https://certbot.eff.org/)
-   [SSL Server Test](https://www.ssllabs.com/ssltest/index.html)
-   [LINE Developers](https://developers.line.me/en/)
-   [Material Icons](https://material.io/tools/icons/?style=baseline)
-   [Material Design Icons](https://materialdesignicons.com/)
-   [UTF-8 Miscellaneous Symbols](https://www.w3schools.com/charsets/ref_utf_misc_symbols.asp)
