#!/bin/bash

BINARY_PATH="./backend/target/release/client"

run_multiple_instances() {
    local count=$1
    for ((i = 1; i <= count; i++)); do
        echo "Starting instance $i"
        # Run the binary in the background
        "$BINARY_PATH" &
    done
}

run_multiple_instances $1

# Prompt to quit all instances
echo "Press Enter to quit all instances"
read -r

pkill -f "$BINARY_PATH"

echo "All instances terminated"
