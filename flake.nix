{
  description = "Deterministic smart contract example";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inputs = [ rust ];
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "smart-contract";
          version = "0.0.1";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = inputs;

          buildPhase = ''
            cargo build --release --target=wasm32-unknown-unknown
          '';
          installPhase = "mkdir -p $out; cp -f target/wasm32-unknown-unknown/release/smart_contract.wasm $out/";
        };


        devShell = pkgs.mkShell { packages = inputs; };
      }
    );
}
