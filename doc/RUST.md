## Rust

-   Install rustup

```bash
curl https://sh.rustup.rs -sSf | sh
```

-   Add to ~/.zshrc

```bash
source $HOME/.cargo/env
export RUSTFLAGS="-Aproc-macro-derive-resolution-fallback" # https://github.com/diesel-rs/diesel/issues/1785
```

-   Close and reopen your terminal to start using

-   Install rust

```bash
rustup default nightly # or nightly
rustup component add rustfmt-preview
rustup component add rls-preview rust-analysis rust-src
```
