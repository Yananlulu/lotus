## Usage

### build for yourself

```bash
docker build -t pi .
# test
docker run --rm -it -v $HOME/workspace:/workspace -v $HOME/.ssh:/root/.ssh pi:latest
```
