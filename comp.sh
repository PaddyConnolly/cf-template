#!/bin/bash

# Usage: ./comp.sh 800 4a
RATING=$1
PROBLEM=$2

if [ -z "$RATING" ] || [ -z "$PROBLEM" ]; then
  echo "Usage: ./comp.sh <rating> <problem_id> (e.g., ./comp.sh 800 4a)"
  exit 1
fi

# 1. Paths
TARGET_DIR="${RATING}s"
FILE_NAME=$(echo "$PROBLEM" | tr '[:upper:]' '[:lower:]')
TARGET_FILE="$TARGET_DIR/$FILE_NAME.rs"
OUTPUT_BIN="$TARGET_DIR/$FILE_NAME"

# 2. Check if file exists
if [ ! -f "$TARGET_FILE" ]; then
  echo "Error: $TARGET_FILE not found."
  exit 1
fi

# 3. Detect OS and set flags
OS_TYPE=$(uname -s)
case "$OS_TYPE" in
Linux*) FLAGS="-C link-arg=-Wl,-z,stack-size=268435456" ;;
Darwin*) FLAGS="-C link-arg=-Wl,-stack_size,0x10000000" ;;
MINGW*)
  FLAGS="-C link-args=/STACK:268435456"
  OUTPUT_BIN="$OUTPUT_BIN.exe"
  ;;
*) FLAGS="" ;;
esac

echo "Compiling $TARGET_FILE..."

# 4. Compile with 2024 edition and optimizations
rustc --edition=2024 -O $FLAGS --cfg ONLINE_JUDGE "$TARGET_FILE" -o "$OUTPUT_BIN"

# 5. Run if successful
if [ $? -eq 0 ]; then
  echo "Running..."
  echo "-----------------------------------"
  "./$OUTPUT_BIN"
else
  echo "Compilation failed."
fi
