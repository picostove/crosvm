{
  description = "crosvm";

  inputs.nixpkgs.url = "nixpkgs/nixos-22.11";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nix-filter.url = "github:numtide/nix-filter";

  nixConfig = {
    extra-substituters = ["https://rivosinc.cachix.org"];
    extra-trusted-public-keys = ["rivosinc.cachix.org-1:GukvLG5z5jPxRuDu9xLyul0vue1gD1wSChJjljiwpf0="];
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    nix-filter,
  }: let
    inherit (flake-utils) lib;
    version = self.shortRev or "dirty";
    src = nix-filter.lib {
      root = ./.;
      exclude = [
        ./flake.nix
        ./flake.lock
        (nix-filter.lib.matchExt "nix")
        ./.github
      ];
    };
  in
    lib.eachSystem [
      lib.system.aarch64-linux
      lib.system.riscv64-linux
      lib.system.x86_64-linux
    ] (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        packages = lib.flattenTree rec {
          crosvm = pkgs.callPackage ./package.nix {inherit src version;};
          crosvm-aarch64 = pkgs.pkgsCross.aarch64-multiplatform.callPackage ./package.nix {inherit src version;};
          crosvm-riscv64 = pkgs.pkgsCross.riscv64.callPackage ./package.nix {inherit src version;};
          default = crosvm;
        };
        checks = nixpkgs.lib.mapAttrs (name: pkg: pkg.override {doCheck = true;}) (builtins.removeAttrs packages ["default"]);
        formatter = pkgs.alejandra;
      }
    );
}
