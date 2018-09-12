## Usage

### build for yourself

```bash
docker build -t lotus .
# test
docker run --rm -it -p 2222:22 -p 8080:8080 -p 3000:3000 -v $HOME/.ssh:/home/deploy/.ssh -v "$(dirname "$(pwd)")":/workspace lotus:latest
# start(first time)
docker run --name lotus -d -p 2222:22 -p 8080:8080 -p 3000:3000 -v $HOME/.ssh:/home/deploy/.ssh -v "$(dirname "$(pwd)")":/workspace lotus:latest
# stop
docker stop lotus
# start
docker start lotus
# remove
docker rm lotus
```

### How to ssh

```bash
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
cd ~/.ssh
cat id_rsa.pub >> authorized_keys
```

## Documents

-   [Run multiple services in a container](https://docs.docker.com/config/containers/multi-service_container/)
