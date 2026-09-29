{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    binutils
    cargo
    git
    gmp
    m4
    gnumake
    opam
    pkg-config
    rustc
    rustfmt
    clippy
    rust-analyzer
    xonsh
    z3
  ];

  shellHook = ''
    cat <<'EOF'
STAR Binsec analysis shell is ready.

The side-channel check builds:
  x86_64-fortanix-unknown-sgx/release/analysis

and analyzes that ELF with Binsec checkct.

Binsec itself is installed reproducibly by core/analysis/Dockerfile.
For local runs, install Binsec with x86-64 support (unisim_archisec)
and add the Fortanix SGX Rust target before running:

  xonsh analysis/run.xonsh
EOF
  '';
}
