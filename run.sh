#!/bin/bash

# Usage: ./run.sh <day_number> [--sample] [--part=N]
# Example: ./run.sh 1
# Example: ./run.sh 1 --sample
# Example: ./run.sh 1 --part=2
# Example: ./run.sh 1 --sample --part=2

if [ -z "$1" ]; then
  echo "Usage: ./run.sh <day_number> [--sample] [--part=N]"
  echo "Example: ./run.sh 1"
  echo "Example: ./run.sh 1 --sample"
  echo "Example: ./run.sh 1 --part=2"
  exit 1
fi

DAY=$1
PART=1
POSTFIX=""

# Parse optional arguments
shift
for arg in "$@"; do
  case $arg in
    --sample)
      POSTFIX="_sample"
      ;;
    --test)
      POSTFIX="_test"
      ;;
    --part=*)
      PART="${arg#*=}"
      ;;
  esac
done

INPUT_FILE="input/${DAY}${POSTFIX}.txt"

if [ ! -f "$INPUT_FILE" ]; then
  echo "Error: Input file $INPUT_FILE not found"
  exit 1
fi

echo "Running day ${DAY} (part ${PART})..."
cargo run --bin "day${DAY}" --release -- "$INPUT_FILE" "$PART"
