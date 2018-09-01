#!/bin/zsh

curl https://sh.rustup.rs -sSf | sh -s -- -y
echo 'source $HOME/.cargo/env' >> $HOME/.zshrc

source $HOME/.zshrc

rustup override set nightly
rustup component add rls-preview rust-analysis rust-src rustfmt-preview
cargo install --force diesel_cli
