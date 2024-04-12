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
              targets = [ "x86_64-pc-windows-gnu" ];
            };

            crates."acc-tools" = {
              targets."x86_64-pc-windows-gnu" = {
                default = true;
                drvConfig.mkDerivation = {
                  nativeBuildInputs = with pkgs; [ pkgsCross.mingwW64.stdenv.cc ];
                  buildInputs = with pkgs.pkgsCross.mingwW64.windows; [ mingw_w64_pthreads ];
                };
              };

              drvConfig.mkDerivation = {
                nativeBuildInputs = with pkgs; [ pkgsCross.mingwW64.stdenv.cc ];
                buildInputs = with pkgs.pkgsCross.mingwW64.windows; [ mingw_w64_pthreads ];
              };
              runtimeLibs = with pkgs; [
                libxkbcommon
                libGL

                # WINIT_UNIX_BACKEND=wayland
                wayland

                # WINIT_UNIX_BACKEND=x11
                xorg.libXcursor
                xorg.libXrandr
                xorg.libXi
                xorg.libX11

                freetype
                fontconfig
              ];
            };
          };


          devShells.default = crateOutputs.devShell;
          packages.default = crateOutputs.packages.release;
        };
    };
}
