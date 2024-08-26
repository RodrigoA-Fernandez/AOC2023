{pkgs,...}:{
  pkgs.mkshell {
    nativeBuildInputs = with pkgs.buildPackages: [
      cargo
    ];
  }
}
