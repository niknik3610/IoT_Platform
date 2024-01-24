cp -r ../proto .
cargo publish --allow-dirty
rm -rf ./proto
