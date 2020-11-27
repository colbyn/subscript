set -e

rm -rf example/output
cargo run -- serve --root=example --input 'example/pages/**/*.html' --output=example/output --trim pages --open-browser



