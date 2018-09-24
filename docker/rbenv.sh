#!/bin/zsh

set -e

git clone https://github.com/sstephenson/rbenv.git $HOME/.rbenv
git clone https://github.com/sstephenson/ruby-build.git $HOME/.rbenv/plugins/ruby-build
git clone https://github.com/rbenv/rbenv-vars.git $HOME/.rbenv/plugins/rbenv-vars
echo 'export PATH=$HOME/.rbenv/bin:$PATH' >> $HOME/.zshrc
echo 'eval "$(rbenv init -)"' >> $HOME/.zshrc

source $HOME/.zshrc

rbenv install 2.5.1
rbenv global 2.5.1
gem install bundler
gem install rubocop
