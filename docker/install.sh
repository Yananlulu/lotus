#!/bin/zsh -i

set -e

# java
sdk install java
sdk install maven
sdk install gradle

# nodejs
nvm install node

# ruby
rbenv install 2.5.1
rbenv global 2.5.1
gem install bundler
gem install rubocop

# rust
rustup override set nightly
rustup component add rls-preview rust-analysis rust-src rustfmt-preview
cargo install --force diesel_cli
