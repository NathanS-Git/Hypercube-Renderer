#!/usr/bin/env bash
set -euo pipefail

# Render a perfectly looping hypercube video.
#
# Usage:
#   ./render.sh              # 4D hypercube, 6 second loop
#   ./render.sh 5            # 5D hypercube, 6 second loop
#   ./render.sh 5 10         # 5D hypercube, 10 second loop
#   DIM=6 LOOP_SECONDS=4 ./render.sh

DIM="${1:-${DIM:-4}}"
LOOP_SECONDS="${2:-${LOOP_SECONDS:-6}}"

export DIM
export LOOP_SECONDS

echo "Rendering ${DIM}D hypercube, ${LOOP_SECONDS}s perfect loop..."
cargo run --release -- capture
