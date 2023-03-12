#!/bin/bash

trgrep_output_file="rust_output.txt"
grep_output_file="grep_output.txt"

function run_test {
  local options="$1"

  # Run trgrep and redirect its output to a file
  cargo run --quiet -- $options > "$trgrep_output_file"

  # Run the real grep and redirect its output to a file
  grep $options > "$grep_output_file"

  # Compare the two output files using diff
  if diff "$trgrep_output_file" "$grep_output_file" >/dev/null ; then
    echo "$options: passed"
    return 0
  else
    echo "$options: FAILED"
    return 1
  fi
}

# Run some tests
run_test "-v --ignore-case twinkle sample-files/poem.txt"
run_test "-w 'apple' sample-files/words.txt"
run_test "a sample-files/words.txt"
run_test "-n 4 sample-files/numbers.txt"
run_test "-h -n fil src/lib.rs src/main.rs"
run_test "-l { src/lib.rs src/main.rs"
run_test "-c -l l src/lib.rs src/main.rs"

# Clean up
rm -f $trgrep_output_file $grep_output_file
