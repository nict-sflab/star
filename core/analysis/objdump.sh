cd ../
for A in 0x4019884 0x4014fc5 0x4012cd3 0x40121b5 0x4013af9; do
  F=$(printf "0x%x" $((A - 0x4000000)))
  echo "=== A=$A F=$F ==="
  objdump -Cd --demangle ./target/release/analysis \
    --start-address=$((F-96)) \
    --stop-address=$((F+512))
done
