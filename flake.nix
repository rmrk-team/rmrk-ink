{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          fenix.overlays.default
        ];
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs.fenix;
            [ 
              (fromToolchainFile {
                file = ./rust-toolchain.toml;
                sha256 = "sha256-iRN1Cmqr9ZDOgW8EakVHjiPgN5H17bFpTG7W9EOJzhg=";
              })
            ];
        };
      });
}
