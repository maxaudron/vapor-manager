{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs = inputs@{ parts, nci, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];
      imports = [ nci.flakeModule ];
      perSystem = { pkgs, config, lib, ... }:
        let
          # shorthand for accessing this crate's outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          crateOutputs = config.nci.outputs."acc-tools";
        in
        {
          nci = {
            projects."acc-tools".path = ./.;
            toolchainConfig = {
              channel = "stable";
              components = [ "rustfmt" "rust-src" ];
            };
          };


          devShells.default = crateOutputs.devShell;
          packages.default = crateOutputs.packages.release;
        };
    };
}
