FROM archlinux:latest

#---------------------
#        ROOT

ENV CC /usr/bin/clang
ENV CXX /usr/bin/clang++
ENV GYP_GENERATORS ninja

RUN pacman -Syu --needed --noconfirm \
    bash \
    coreutils \
    clang \
    cmake \
    curl \
    debugedit \
    fakeroot \
    git \
    glibc \
    gtk3 \
    jdk17-openjdk \
    ninja \
    pkgconf \
    protobuf \
    rustup \
    sudo \
    unzip \
    which

RUN ln -s /usr/bin/sha1sum /usr/bin/shasum

# Create userspace
RUN useradd -m -s /bin/bash developer && \
    echo 'developer ALL=(ALL) NOPASSWD:ALL' >>/etc/sudoers && \
    sudo chown -R developer:developer /home/developer && \
    sudo chown -R developer:developer /opt

WORKDIR /home/developer
USER developer

#---------------------
#      USERSPACE

ENV HOME "/home/developer"
ENV PATH "$PATH:$HOME/flutter/bin"
ENV PATH "$PATH:$HOME/.pub-cache/bin"
ENV PATH "$PATH:$HOME/.cargo/bin"

# Rust
RUN rustup default stable

# Yay
RUN git clone https://aur.archlinux.org/yay-bin.git && \
    cd yay-bin/ && \
    makepkg -si --noconfirm && \
    rm -rf yay-bin

# Android SDK
RUN yay -Sy --needed --noconfirm \
    android-sdk-cmdline-tools-latest \
    android-sdk \
    android-ndk

ENV ANDROID_HOME "/opt/android-sdk"
ENV ANDROID_NDK_HOME "/opt/android-ndk"
ENV PATH "$PATH:/opt/android-sdk/cmdline-tools/latest/bin/"

RUN yes | sdkmanager --licenses

# Flutter
RUN git clone https://github.com/flutter/flutter.git

RUN flutter config --disable-analytics >/dev/null && \
    flutter config --enable-native-assets && \
    flutter doctor && \
    flutter precache --linux

# More Onigiri
RUN mkdir -p moreonigiri && \
    sudo chown -R developer:developer moreonigiri

WORKDIR /home/developer/moreonigiri

CMD echo && \
    clear && \
    echo -e "❤️ \033[1m\033[33mWelcome to the More Onigiri dev shell!\033[0m ❤️\n" && \
    /bin/bash
