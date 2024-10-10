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
            cargo
            flutter
            rustc                        
            #-----linux-----
            cmake
            graphite2
            gtk3
          ];
          
          shellHook = ''
            export CC=clang
            export CXX=clang++
            export PATH="$PATH:${pkgs.flutter}/bin"
            export FLUTTER_SDK="$FLUTTER_HOME"

            flutter config --no-analytics > /dev/null
          '';
        };
      });
}