#!/bin/bash

# Usage: ./init.sh <rating> <problem_id>
# Example: ./init.sh 800 4a

RATING=$1
PROBLEM=$2

if [ -z "$RATING" ] || [ -z "$PROBLEM" ]; then
  echo "Usage: ./init.sh <rating> <problem_id> (e.g., ./init.sh 800 4a)"
  exit 1
fi

TARGET_DIR="${RATING}s"
FILE_NAME=$(echo "$PROBLEM" | tr '[:upper:]' '[:lower:]')
TARGET_PATH="$TARGET_DIR/$FILE_NAME.rs"

mkdir -p "$TARGET_DIR"

if [ -f "solution.rs" ]; then
  cp --update=none "solution.rs" "$TARGET_PATH"
else
  echo "Error: solution.rs template not found!"
  exit 1
fi

# ---- Cargo.toml bin generation ----

BIN_NAME="$FILE_NAME"
BIN_PATH="$TARGET_PATH"

# Check if bin already exists
if ! grep -q "path = \"$BIN_PATH\"" Cargo.toml; then
  cat >>Cargo.toml <<EOF

[[bin]]
name = "$BIN_NAME"
path = "$BIN_PATH"
EOF
fi

# ----------------------------------

nvim "$TARGET_PATH"
