{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";

    # flake modules
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";

    # Rust
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.process-compose-flake.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          config,
          self',
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          overlays = [ inputs.rust-overlay.overlays.default ];

          src = lib.cleanSource ./.;
          buildInputs = [ ];
          nativeBuildInputs = [
            # Compiler
            rust
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli
            pkgs.binaryen

            # LSP
            pkgs.nil

            # Other Tools
            pkgs.rainfrog
            self'.packages.database-runner
          ];
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src buildInputs nativeBuildInputs;
          };

          poopoo = craneLib.buildPackage {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;

            strictDeps = true;
            doCheck = true;

            meta = {
              licenses = [ lib.licenses.mit ];
            };
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;

            cargoClippyExtraArgs = "--verbose -- --deny warnings";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          process-compose.database-runner =
            { config, ... }:
            let
              dbName = "poopooDatabase";
            in
            {
              imports = [
                inputs.services-flake.processComposeModules.default
              ];

              services.postgres."pg1" = {
                enable = true;
                port = 5432;
              };

              settings.processes.pgweb =
                let
                  pgcfg = config.services.postgres.pg1;
                in
                {
                  environment.PGWEB_DATABASE_URL = pgcfg.connectionURI { inherit dbName; };
                  command = pkgs.pgweb;
                  depends_on."pg1".condition = "process_healthy";
                };

              settings.processes.test = {
                command = pkgs.writeShellApplication {
                  name = "pg1-test";
                  runtimeInputs = [ config.services.postgres.pg1.package ];
                  text = ''
                    echo 'SELECT version();' | psql -h 127.0.0.1 ${dbName}
                  '';
                };
                depends_on."pg1".condition = "process_healthy";
              };
            };

          treefmt = {
            projectRootFile = "flake.nix";

            # Nix
            programs.nixfmt.enable = true;

            # Rust
            programs.rustfmt.enable = true;
            settings.formatter.rustfmt.command = "${rust}/bin/rustfmt";

            # TOML
            programs.taplo.enable = true;

            # GitHub Actions
            programs.actionlint.enable = true;

            # Markdown
            programs.mdformat.enable = true;

            # ShellScript
            programs.shellcheck.enable = true;
            programs.shfmt.enable = true;
          };

          packages = {
            inherit poopoo;
            default = poopoo;
            doc = cargo-doc;
          };

          checks = {
            inherit cargo-clippy;
          };

          devShells.default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;

            inputsFrom = [
              config.process-compose.database-runner.services.outputs.devShell
            ];

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
