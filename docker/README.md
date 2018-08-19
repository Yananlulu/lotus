## Usage

### docker

```bash
docker build -t lotus .
# test
docker run --rm -it -p 2222:22 -p 8080:8080 -p 3000:3000 -v "$(dirname "$(pwd)")":/app lotus:latest
# start(first time)
docker run --name lotus -d -p 2222:22 -p 8080:8080 -p 3000:3000 -v "$(dirname "$(pwd)")":/app lotus:latest
# stop
docker stop lotus
# start
docker start lotus
# remove
docker rm lotus
```

-   ssh

```bash
cd /app
cargo run
cd dashboard
```
