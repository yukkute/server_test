#!/usr/bin/env python3

import clean
import protoc
import mobx
import flutter
import rust


def main():
    c = "\033[1;97m"
    r = "\033[0m"
    print(f"🏗️ {c}Executing build script...{r}\n")

    clean.clean_build_directories(ask=True)
    protoc.generate_protobuf_files()
    mobx.compile_mobx()
    flutter.build_flutter_client()
    rust.build_rust_server()
    rust.copy_shared_lib()

    print(f"\n🌈 {c}Build complete!{r}\n")


if __name__ == "__main__":
    main()
