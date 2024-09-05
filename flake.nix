{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.lib.${system};
    in rec {
      packages.default = packages.hyprsome;
      packages.hyprsome = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
      };

      devShells.default = craneLib.devShell {
        packages = with pkgs; [
          rustfmt
        ];
      };
    });
}
