#!/bin/zsh

curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | sh
echo 'export NVM_DIR="$HOME/.nvm"' >> $HOME/.zshrc
echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> $HOME/.zshrc

source $HOME/.zshrc

nvm install node
