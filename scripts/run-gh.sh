set -e

rm -rf school-notes/output
cargo run -- compile \
    --root=school-notes \
    --input 'school-notes/pages/**/*.html' \
    --output=school-notes/output \
    --trim pages \
    --base-url 'https://colbyn.github.io/school-notes'


