# LOTUS

A complete open source Office-Automation solution.

## Usage

-   Clone project

```bash
cargo install --force diesel_cli --no-default-features --features "postgres mysql"

npm install && cd dashboard && npm install --no-save && cd -
cd lotus
make # dist.tar.xz
```

-   Install [docker](doc/DOCKER.md), [rust](doc/RUST.md), [ruby](doc/RUBY.md), [nodejs](doc/NODEJS.md)

-   Start docker

```bash
git clone https://github.com/saturn-xiv/lotus.git # or your repo
cd lotus
docker pull chonglou/lotus:latest
docker run --name lotus -d -p 2222:22 -p 8080:8080 -p 3000:3000 -p 6379:6379 -p 5432:5432 -p 3306:3306 -p 5672:5672 -p 15672:15672 -v "$(pwd)":/app chonglou/lotus:latest # ONLY FOR FIRST TIME
docker start lotus # next time
```

-   Login to docker

```bash
ssh -p 2222 deploy@localhost # password is 123456
> cd /app # is your work folder
```

-   Install dependencies

```bash
cargo build
npm run install
cd dashboard && npm run install && cd -
cd tools && bundle install && cd -
```

-   Database

```bash
cd tools
rake db:create # creates database
rake db:drop # dorps database
rake db:migrate # migrate database
```

-   Development

```bash
# backend
cargo build
./target/debug/lotus generate:config # generate config.toml file
cargo run # http://localhost:8080
# for frontend
cd dashboard
npm run start # http://localhost:3000
```

-   Deployment

```bash
> make
> ls -l dist
```

## Notes

-   Generate a random key

```bash
openssl rand -base64 32
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
