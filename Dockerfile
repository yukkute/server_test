FROM archlinux:latest

#----- ROOT -----

RUN pacman -Syu --needed --noconfirm \
bash \
coreutils \
clang \
cmake \
curl \
#dart \
debugedit \
fakeroot \
fontconfig \
freetype2 \
gcc-libs \
git \
glibc \
gtk3 \
jdk17-openjdk \
libx11 \
libxext \
libxrender \
libxtst \
llvm \
lld \
ncurses \
ninja \
pkgconf \
protobuf \
rustup \
sudo \
unzip \
xz \
wget \
which \
zlib \
#--multilib--
lib32-gcc-libs \
lib32-glibc

# Linux flutter
RUN ln -s /usr/bin/sha1sum /usr/bin/shasum
ENV CC=/usr/bin/clang
ENV CXX=/usr/bin/clang++
ENV GYP_GENERATORS=ninja

# User
RUN useradd -m -s /bin/bash developer
RUN echo 'developer ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN sudo chown -R developer:developer /home/developer
WORKDIR /home/developer
USER developer

#----- Developer -----

# PATH
ENV HOME="/home/developer"
ENV PATH="$PATH:$HOME/flutter/bin"
ENV PATH="$PATH:$HOME/.pub-cache/bin"
ENV PATH="$PATH:$HOME/.cargo/bin"

# Rustup in user
RUN rustup default stable
RUN cargo install cargo-expand
RUN cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0'

# Yay
RUN git clone https://aur.archlinux.org/yay-bin.git && \
    cd yay-bin/ && makepkg -si --noconfirm && rm -rf yay-bin

# Android SDK
RUN yay -Sy --needed --noconfirm \
android-sdk-cmdline-tools-latest \
android-sdk \
android-ndk
#android-sdk-build-tools

RUN sudo chown -R developer:developer /opt

ENV ANDROID_HOME /opt/android-sdk
ENV ANDROID_NDK_HOME /opt/android-ndk
ENV PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"


RUN yes | sudo sdkmanager --licenses

# Flutter
RUN git clone https://github.com/flutter/flutter.git
RUN dart --disable-analytics
RUN flutter --disable-analytics
RUN flutter doctor

# More Onigiri
RUN flutter config --enable-native-assets
RUN mkdir -p moreonigiri

WORKDIR /home/developer/moreonigiri
RUN sudo chown -R developer:developer .
