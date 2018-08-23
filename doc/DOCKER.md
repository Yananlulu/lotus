## DOCKER

-   [Document](https://docs.docker.com/install/)

-   Install(for archlinux)

```bash
sudo pacman -S docker
sudo systemctl enable docker
sudo gpasswd -a who-am-i docker
```

```bash
docker image prune # remove all dangling images
docker ps
```
