set -e

cargo install --path climsg  --offline || cargo install --path climsg
cargo install --path climsgd --offline || cargo install --path climsgd

echo "Done."
