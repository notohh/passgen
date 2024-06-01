{
  inputs = {
    systems.url = "github:nix-systems/default-linux";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    self,
    systems,
    nixpkgs,
    fenix,
    pre-commit-hooks,
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    pkgsFor = eachSystem (system:
      import nixpkgs {
        localSystem.system = system;
        overlays = [fenix.overlays.default];
      });
  in {
    checks = eachSystem (system: {
      pre-commit-check = pre-commit-hooks.lib.${system}.run {
        src = ./.;
        hooks = {
          # rust
          rustfmt.enable = true;
          taplo.enable = true;
          # nix
          statix.enable = true;
          alejandra.enable = true;
          deadnix.enable = true;
        };
      };
    });
    devShells = eachSystem (system: {
      default = pkgsFor.${system}.mkShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;
        packages = with pkgsFor.${system}; [
          nil
          alejandra
          pkg-config
          openssl
          cargo-modules
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
            "rust-analyzer"
          ])
        ];
        RUST_BACKTRACE = 1;
        RUST_SRC_PATH = "${fenix.packages.${system}.complete.rust-src}/lib/rustlib/src/rust/library";
      };
    });
  };
}
