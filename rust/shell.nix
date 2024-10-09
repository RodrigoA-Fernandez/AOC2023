let
  pkgs = import <nixpkgs> {};
  fenix = pkgs.callPackage "${
      fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"
    }/packages.nix" { };
in
  pkgs.mkShell {
    buildInputs = [
      fenix.default.rustc
      pkgs.rustanalyzer
    ];
  }

