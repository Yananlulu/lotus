## Rust

-   Install rustup

```bash
curl https://sh.rustup.rs -sSf | sh
```

-   Add to ~/.zshrc

```bash
source $HOME/.cargo/env
```

-   Close and reopen your terminal to start using

-   Install rust

```bash
rustup default stable # or nightly
rustup component add rustfmt-preview
rustup component add rls-preview rust-analysis rust-src
```
