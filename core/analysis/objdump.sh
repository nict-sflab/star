cd ../
for A in 0x401C544 0x4014DB5 0x4017C85 0x40166C9 0x40158D5; do
  F=$(printf "0x%x" $((A - 0x4000000)))
  echo "=== A=$A F=$F ==="
  objdump -Cd --demangle ./target/release/analysis \
    --start-address=$((F-96)) \
    --stop-address=$((F+512))
done
