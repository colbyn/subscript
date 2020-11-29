set -e

CARGO_FLAGS=""

if [ -z "$1" ]
then
    # NOT DEFINED
    CARGO_FLAGS+=""
else
    # OTHERWISE - DEFINED
    CARGO_FLAGS+="$1"
fi


rm -rf school-notes/output
cargo run $CARGO_FLAGS -- serve \
    --root=school-notes \
    --input 'school-notes/pages/**/*.html' \
    --output=school-notes/output \
    --trim pages \
    --open-browser



