#!/bin/zsh

set -e

curl -s "https://get.sdkman.io" | zsh
sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config

source $HOME/.zshrc

sdk install java 8.0.181-oracle
sdk install maven
sdk install gradle
