{
  description = "Minimal Nix flake with Flutter, Rust, and protoc";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    #flutter.url = "github:flutter/flutter";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = false;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        dart
        flutter
        rustc
        #-----linux-----
        cmake
        gtk3
      ];
      
      shellHook = ''
        flutter config --no-analytics > /dev/null
      '';
    };
  };
}
