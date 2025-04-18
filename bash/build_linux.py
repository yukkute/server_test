#!/usr/bin/env python3

import clean
import dart_build_runner
import flutter
import rust


def main():
    c = "\033[1;97m"
    r = "\033[0m"
    print(f"🏗️ {c}Executing build script...{r}\n")

    clean.clean_build_directories(ask=True)
    dart_build_runner.run_build_runner()
    flutter.build_flutter_client()
    rust.build_rust_server()
    rust.copy_shared_lib()

    print(f"\n🌈 {c}Build complete!{r}\n")


if __name__ == "__main__":
    main()
