set -e

rm -rf school-notes/output
cargo run -- serve \
    --root=school-notes \
    --input 'school-notes/pages/**/*.html' \
    --output=school-notes/output \
    --trim pages \
    --open-browser



