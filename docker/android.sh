#!/bin/zsh

set -e

export ANDROID_SDK_HOME=$HOME/local/android-sdk
export ANDROID_SDK_VERSION=4333796

mkdir -p $HOME/downloads
cd $HOME/downloads
wget https://dl.google.com/android/repository/sdk-tools-linux-$ANDROID_SDK_VERSION.zip
mkdir -p $ANDROID_SDK_HOME
unzip sdk-tools-linux-$ANDROID_SDK_VERSION.zip -d $ANDROID_SDK_HOME
yes | $ANDROID_SDK_HOME/tools/bin/sdkmanager --licenses
