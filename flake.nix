{
  description = "Bevywind UI style macros and language server";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forEachSystem = f:
        nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forEachSystem (pkgs: {
        bevywind-lsp = pkgs.rustPlatform.buildRustPackage {
          pname = "bevywind-lsp";
          version = "0.1.0";
          src = ./.;

          cargoBuildType = "release";

          cargoLock = {
            lockFile = ./bevywind-lsp/Cargo.lock;
          };

          cargoBuildFlags = [ "--manifest-path" "bevywind-lsp/Cargo.toml" ];
          cargoTestFlags = [ "--manifest-path" "bevywind-lsp/Cargo.toml" ];

          meta.mainProgram = "bevywind-lsp";
        };

        default = self.packages.${pkgs.stdenv.hostPlatform.system}.bevywind-lsp;
      });

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            pkg-config
            clang
          ];
        };
      });
    };
}
