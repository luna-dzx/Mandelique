{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    SDL2
    cmake
    cargo
    shaderc
    wayland
    wayland-protocols
    wayland-utils
    libxkbcommon
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    pkg-config
  ];

  shellHook = ''
    export WINIT_UNIX_BACKEND=wayland
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:${pkgs.vulkan-loader}/lib/
    #export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXi}/lib

    '';
}
