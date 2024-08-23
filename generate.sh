#!/bin/bash

if [ -z "$1" ]; then
  echo "Required generator file is missing."
  exit 1
fi

file="$1"

# Function to remove lines between start and end markers
remove_lines_between_markers() {
    local file=$1
    local start_marker=$2
    local end_marker=$3

    # Use sed to remove lines between start and end markers, inclusive
    sed -i '' -e "/$start_marker/,/$end_marker/d" "$file"
}

# Function to process a single Rust file
process_rust_file() {
    local file=$1

    # Temporary file to store line numbers for start and end markers
    tmp_file=$(mktemp)

    # Find lines that start with 'impl ' and don't contain 'fmt::Display'
    grep -n '^impl ' "$file" | grep -vE 'Configuration|fmt::Display|fmt::Debug|From<' > "$tmp_file"

    # Read line numbers and process each block in reverse order
    tac "$tmp_file" | while IFS= read -r line; do
        # Extract line number from grep output
        start_line=$(echo "$line" | cut -d: -f1)

        # Find the end line (next line with only a '}')
        end_line=$(awk "NR>$start_line && /^}$/ {print NR; exit}" "$file")

        if [[ -n "$end_line" ]]; then
            # Build the marker strings
            start_marker="^$(sed "${start_line}q;d" "$file")$"
            end_marker="^$(sed "${end_line}q;d" "$file")$"

            # Remove lines between markers
            remove_lines_between_markers "$file" "$start_marker" "$end_marker"
        fi
    done

    # Clean up temporary file
    rm "$tmp_file"

    # Remove 'Default' from #[derive(...)] lines
    sed -i '' -E 's/(\#\[derive\([^\)]*)Default,?\s*/\1/' "$file"
}

# Function to recursively process all Rust files in a directory
process_directory() {
    local dir=$1

    # Find all Rust files and process them
    find "$dir" -type f -name "*.rs" | while IFS= read -r rust_file; do
        echo "Processing $rust_file..."
        process_rust_file "$rust_file"
    done
}

# Remove codegen output from previous run
rm -rf ./openapi

# Run codegen
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli:latest generate \
  -i "/local/$file" \
  -g rust \
  -o /local/openapi \
  --additional-properties=packageVersion=0.1.0

# Apply codemod to codegen
process_directory "openapi"


# Move the generated ./openapi/src directory to crate source directory
mv ./openapi/src/* ./src/openapi

# Remove unused codegen output
rm -rf ./openapi

# Rename lib.rs to mod.rs, to indicate it's a module
mv ./src/openapi/lib.rs ./src/openapi/mod.rs

# Add clippy & dead_code configuration to mod.rs
tmpfile=$(mktemp)
echo -e "#![allow(clippy::all)]\n#![allow(dead_code)]" > "$tmpfile"
cat ./src/openapi/mod.rs >> "$tmpfile"
mv "$tmpfile" ./src/openapi/mod.rs

sed -i '' -E 's/crate::/crate::openapi::/g' ./src/openapi/**/*.rs
