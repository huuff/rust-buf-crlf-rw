{
  description = "Rust library to read/write CRLF-delimited strings from/to buffers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, rust-overlay, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
  in with pkgs;
  {
    devShell.${system} = mkShell {
      buildInputs = [
        (rust-bin.beta.latest.default.override {
          extensions = [ "rust-src" ];
        })
        cargo
        rust-analyzer
      ];
    };
  };
}
