FROM archlinux:latest

RUN pacman -Syu --noconfirm curl git sudo unzip

# Rust
RUN pacman -S --noconfirm rustup
RUN rustup default stable

# Flutter
RUN pacman -S --noconfirm xz wget which
RUN git clone https://github.com/flutter/flutter.git
ENV PATH="$PATH:/flutter/bin"
RUN flutter

# Linux flutter
RUN pacman -S --noconfirm coreutils clang cmake gtk3 ninja pkgconf
RUN ln -s /usr/bin/sha1sum /usr/bin/shasum
ENV CC=/usr/bin/clang
ENV CXX=/usr/bin/clang++
ENV GYP_GENERATORS=ninja

# PKGBUILD
RUN pacman -S --noconfirm debugedit fakeroot

# User
RUN useradd -m -s /bin/bash developer
RUN echo 'developer ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER developer
WORKDIR /moreonigiri

# Yay
RUN git clone https://aur.archlinux.org/yay-bin.git && \
    cd yay-bin/ && makepkg -si --noconfirm

RUN rm -rf yay-bin

# Android SDK
RUN yay -Sy --needed --noconfirm \
android-sdk \
android-sdk-build-tools \
android-sdk-cmdline-tools-latest \
android-platform \
android-sdk-platform-tools
ENV ANDROID_HOME /opt/android-sdk
ENV PATH $PATH:$ANDROID_HOME/cmdline-tools/latest/bin
RUN yes | sdkmanager --licenses

# Flutter doctor
RUN sudo chown -R developer:developer /flutter
RUN yes | flutter doctor --android-licenses

# More Onigiri dependencies
RUN sudo flutter config --enable-native-assets
RUN sudo pacman -S --noconfirm llvm lld

COPY . .