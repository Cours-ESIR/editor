{
  description = "HyperAST";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            (rust-bin.fromRustupToolchainFile ./src-tauri/rust-toolchain)
            pkg-config
            nixfmt
            cmake
            imv

            nodejs_21
            cargo-tauri

            webkitgtk
            libsoup.dev
            openssl
          ];
          libraries = with pkgs; [
            openssl
            glibc
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
        };
      }
    );
}