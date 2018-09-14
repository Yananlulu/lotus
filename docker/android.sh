#!/bin/zsh

export ANDROID_SDK_HOME=$HOME/local/android-sdk

mkdir -p $ANDROID_SDK_HOME
wget  -qO- https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip | bsdtar -xvf- -C $ANDROID_SDK_HOME
yes | $ANDROID_SDK_HOME/tools/bin/sdkmanager --licenses
