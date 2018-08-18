## Usage

-   Install [docker](https://docs.docker.com/install/)

-   Build

```bash
docker build -t lotus .
```

-   Start

```bash
docker run --rm -it -p 28080:8080 -p 25432:5432 -p 26379:6379 -p 25672:15672 lotus:latest
```


