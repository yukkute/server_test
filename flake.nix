{
  description = "Minimal Nix flake with Flutter, Rust, and protoc";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: {
    devShells.default = import nixpkgs {
      system = "x86_64-linux";
    } .mkShell {
      buildInputs = with pkgs; [
        flutter
        rustc
        cargo
        protobuf
      ];
    };
  };
}
