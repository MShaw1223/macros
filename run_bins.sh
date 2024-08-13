# CD project dir
cd "$(dirname "$0")"
# iter through each .rs file
for bin in src/bin/*.rs; do
    echo "
    --- --- --- --- --- --- --- --- ---
    Running $bin...

    "
    binary_name=$(basename "$bin" .rs)
    cargo run --bin "$binary_name"
    echo "

    Finished running $bin
    --- --- --- --- --- --- --- --- ---"
done
