{
  description = "Minimal Nix flake with Flutter, Rust, and protoc";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = false;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            #dart
            rustup
            protobuf
          ];
          
          shellHook = ''
            #export PATH=$PATH:${pkgs.dart}/bin
            export PATH=$PATH:${pkgs.rustup}/bin
            export PATH=$PATH:${pkgs.protobuf}/bin
            echo "Development environment ready!"
          '';
        };
      });
}
