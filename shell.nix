{ pkgs ? import <nixpkgs> { } }:

let
  libDeps = with pkgs; [
    libxkbcommon
    xorg.libXcursor
    xorg.libX11
  ];
in
pkgs.mkShell {
  packages = libDeps;

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libDeps}";
}
