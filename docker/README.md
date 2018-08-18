## Usage

-   Install [docker](https://docs.docker.com/install/)

-   Build

```bash
docker build -t lotus .
```

-   Start

```bash
docker run --rm -it -p 2222:22 -p 8080:8080 -p 3000:3000 -v ..:/app lotus:latest
```
