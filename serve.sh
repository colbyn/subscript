set -e

rm -rf example/output
cargo run --release -- serve --root=example --input 'example/pages/**/*.html' --output=example/output --trim pages



