# LOTUS

A complete open source Office-Automation solution.

## Usage

-   Install [docker](doc/DOCKER.md), [rust](doc/RUST.md), [nodejs](doc/NODEJS.md)

-   Start docker

```bash
git clone https://github.com/saturn-xiv/lotus.git $HOME/workspace/lotus # or your fork repo
cd lotus
docker pull chonglou/lotus:latest
# first startup
docker run --name lotus -d -p 2222:22 -p 8080:8080 -p 3000:3000 -v $HOME/.ssh:/home/deploy/.ssh -v $HOME/workspace:/workspace chonglou/lotus:latest
# next time startup
docker start lotus
```

-   login to docker

```bash
ssh -p 2222 deploy@localhost # default password is 'hi'
> cd /workspace/lotus # is your work folder
```

-   install dependencies packages

```bash
make init
```

-   Development

```bash
> cargo build
> ./target/debug/lotus generate:config # generate config.toml file
> ./target/debug/lotus database:migrate # create database tables
> cargo run # http://localhost:8080
```

-   Start frontend server

```bash
cd dashboard
npm run start # http://localhost:3000
```

-   Deployment

```bash
> make # dist.tar.xz
```
