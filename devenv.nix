{ pkgs, ... }:

{
  packages = with pkgs; [
    rust-analyzer
    wasm-pack
  ];

  languages.rust.enable = true;

  languages.rust.toolchain.rustc = (pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  });
}
