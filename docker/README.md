## Usage

### build for yourself

```bash
docker build -t lotus .
# test
docker run --rm -it -p 2222:22 -p 8080:8080 -p 3000:3000 -p 6379:6379 -p 5432:5432 -p 5672:5672 -p 15672:15672 -v "$(dirname "$(pwd)")":/app lotus:latest
# start(first time)
docker run --name lotus -d -p 2222:22 -p 8080:8080 -p 3000:3000 -p 6379:6379 -p 5432:5432 -p 5672:5672 -p 15672:15672 -v "$(dirname "$(pwd)")":/app lotus:latest
# stop
docker stop lotus
# start
docker start lotus
# remove
docker rm lotus
```

## Documents

-   [Run multiple services in a container](https://docs.docker.com/config/containers/multi-service_container/)
