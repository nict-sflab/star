cd ../
for A in 0x40100F5 0x401256F 0x4012A33 0x40178F4; do
  F=$(printf "0x%x" $((A - 0x4000000)))
  echo "=== A=$A F=$F ==="
  objdump -Cd --demangle ./target/release/analysis \
    --start-address=$((F-96)) \
    --stop-address=$((F+512))
done
