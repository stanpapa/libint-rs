{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        toolchain = with fenix.packages.${system}; combine [
          stable.toolchain
          targets.wasm32-unknown-unknown.stable.rust-std
        ];
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          config.allowUnfree = true;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "libint";

          nativeBuildInputs = with pkgs; [
            # Build / configure tools (executed on the host)
            # pkg-config needs to be in nativeBuildInputs for its setupHooks to run (https://discourse.nixos.org/t/how-can-we-create-shell-script-with-pkg-config-path-defined-for-given-packages/68928)
            pkg-config
            meson
            ninja
            boost
            eigen
            libint
            # (libint.override { enableFortran = false; eriDeriv = 0; eri2Deriv = 0; eri3Deriv = 0; enableOneBody = true; multipoleOrd = 0; enableGeneric = false; })

            # Compilers / toolchain
            toolchain
            llvmPackages.bintools
          ];

          buildInputs = with pkgs; [
            cargo-expand
            cargo-release
            clang-tools
          ];

          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc
          ];

          NIX_LD = builtins.readFile "${pkgs.stdenv.cc}/nix-support/dynamic-linker";

          shellHook = ''
            export LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH:$src/lib
          '';
        };
    });
}
