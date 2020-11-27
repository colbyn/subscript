set -e

rm -rf docs
rm -rf school-notes/output-release

rm -rf school-notes/output
cargo run -- compile \
    --root school-notes \
    --input 'school-notes/pages/**/*.html' \
    --output school-notes/output-release \
    --trim pages \
    --base-url https://colbyn.github.io/subscript/

mv school-notes/output-release docs

git add docs
git commit -m "update site"
git push

