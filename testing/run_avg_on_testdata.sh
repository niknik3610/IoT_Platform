#!/bin/bash

test_dir="./test_data"
script="calculate_avg_from_data.py"

for filename in "$test_dir"/*; do
    python "$script" "$filename"
done
