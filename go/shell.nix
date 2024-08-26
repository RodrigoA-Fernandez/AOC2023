with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    go
    zsh
  ];
  shellHooks = ''
  '';
}
