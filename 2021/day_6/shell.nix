{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustc
    cargo
    rust-analyzer
    clippy
    rustfmt
  ];
}
