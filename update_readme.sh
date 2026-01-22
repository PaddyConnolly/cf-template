#!/bin/bash
set -e

# Find rating directories matching "*s" and sort numerically
dirs=$(find . -maxdepth 1 -type d -name "*s" | sed 's|./||' | sort -n)

declare -A counts

for d in $dirs; do
  # Remove non-.rs files
  for f in "$d"/*; do
    [ -e "$f" ] || continue
    if [[ ! "$f" =~ \.rs$ ]]; then
      echo "Deleting non-.rs file in $d: $(basename "$f")"
      rm -f -- "$f"
    fi
  done

  # Count .rs files
  counts["$d"]=$(ls "$d"/*.rs 2>/dev/null | wc -l)
done

# Build progress text only for dirs with .rs files
progress=""
for d in $dirs; do
  if [ "${counts[$d]}" -gt 0 ]; then
    progress+="${counts[$d]}x$d "
  fi
done

# Update README: replace lines under ## Current Progress
if grep -q "^## Current Progress" README.md; then
  awk -v prog="$progress" '
        BEGIN {skip=0}
        /^## Current Progress/ {print; print ""; print prog; print ""; skip=1; next}
        /^## / {skip=0}
        {if(!skip) print}
    ' README.md >README.tmp
  mv README.tmp README.md
else
  # Fallback if header not found
  echo -e "\n## Current Progress\n\n$progress\n" >>README.md
fi

# Output only the relevant progress line
echo "$progress"
