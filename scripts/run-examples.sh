#!/bin/bash

# The current cargo command does not address this issue yet
# hence the need for this script.
#
# Issue: https://github.com/rust-lang/cargo/issues/8356

# Redirect stderr (2) to stdout (1) and assign to the variable
cmd_output="$(cargo run --example 2>&1)"

# Exclude extra error text and get only example names, which start from 3rd line onwards
count=1
# IFS= means that string splitting occurs at newlines only
# read - reads a single line from the input
while IFS= read -r line
do
    if (( count > 2 )); then
        # Trim any whitespace before or after the string
        example_name=$(echo "$line" | xargs)
        cargo run --example "$example_name"
    fi

    (( count++ ))
# The varible gets printed with a newline and fed into the while loop
done < <(printf '%s\n' "$cmd_output")
