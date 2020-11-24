set -e

rm -rf example/output
cargo run -- compile --root=example --input 'example/pages/**/*.html' --output=example/output --trim pages --base-url 'https://colbyn.github.io/school-notes'


