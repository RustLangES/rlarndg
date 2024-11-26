{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
      fenix = inputs.fenix.packages;
    in
    # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    (flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;
        # Toolchain
        toolchain = fenix.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
        };
        craneLib = crane.overrideToolchain toolchain;

        buildInputs = with pkgs; [
          pkg-config
          openssl
        ] ++ lib.optionals stdenv.isDarwin [
          libiconv
          darwin.apple_sdk.frameworks.Security
        ];

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type:
            (pkgs.lib.hasInfix "/.sqlx" path)
            || (pkgs.lib.hasInfix "/migrations" path)
            || (craneLib.filterCargoSources path type);
        };
        commonArgs = {
          doCheck = false;
          inherit src buildInputs;
        };

        libraries = [ ];
        # Compile all artifacts
        appDeps = craneLib.buildDepsOnly commonArgs;

        # Compile
        app = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = appDeps;
          nativeBuildInputs = (commonArgs.nativeBuildInputs or [ ]) ++ [
            pkgs.sqlx-cli
          ];

          preBuild = ''
            sqlx prepare
          '';
        });
      in {
        # nix build
        packages.default = app;

        # nix run
        apps.default = flake-utils.lib.mkApp {
          drv = app;
        };

        # nix develop
        devShells.default = craneLib.devShell {
          inherit buildInputs;

          packages = with pkgs; [
            toolchain
            pkg-config
            openssl
            sqlx-cli
          ];

          LITCRYPT_ENCRYPT_KEY = "test-key";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
        };
      }
    ));
}
