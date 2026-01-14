#!/bin/bash

# Usage: ./init.sh <rating> <problem_id>
# Example: ./init.sh 800 4a

RATING=$1
PROBLEM=$2

if [ -z "$RATING" ] || [ -z "$PROBLEM" ]; then
  echo "Usage: ./init.sh <rating> <problem_id> (e.g., ./init.sh 800 4a)"
  exit 1
fi

# 1. Define the directory (e.g., 800s)
TARGET_DIR="${RATING}s"

# 2. Normalize file name to lowercase (4A -> 4a.rs)
FILE_NAME=$(echo "$PROBLEM" | tr '[:upper:]' '[:lower:]')
TARGET_PATH="$TARGET_DIR/$FILE_NAME.rs"

# 3. Create the directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# 4. Copy template (solution.rs) to target
if [ -f "solution.rs" ]; then
  # none ensures we don't overwrite if you accidentally run it twice on the same problem
  cp --update=none "solution.rs" "$TARGET_PATH"
else
  echo "Error: solution.rs template not found!"
  exit 1
fi

# 5. Open in Neovim
nvim "$TARGET_PATH"
