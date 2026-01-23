export NIXPKGS_ALLOW_UNFREE=1
nix-shell -p wasm-pack lld --run "wasm-pack build --dev --target web --out-dir ../extension/wasm"