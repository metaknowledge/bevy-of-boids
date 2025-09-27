# works with nix-shell as well
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "Straight Line";
  src = ./.;
  system = "x86_64-linux";
  
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    alsa-oss
    rustc
    cargo
    rustfmt
    openssl
    bacon
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  # LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  RUST_BACKTRACE = 1;
  shellHook = ''
  '';

  buildPhase = "cargo build";

  installPhase = ''
    echo $out
    mkdir -p $out
    mv ./target/debug/bevy-of-boids $out/bevy-of-boids
  '';
}


