{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	packages = with pkgs; [
		cargo
                coreutils
                dart
                flutter
                protobuf
                protoc-gen-dart
                rustc
                which
	];
}
