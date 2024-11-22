#!/bin/bash

# The current cargo command does not address this issue yet
# hence the need for this script.
#
# Issue: https://github.com/rust-lang/cargo/issues/8356

# Redirect stderr (2) to stdout (1) and assign to the variable
cmd_output="$(cargo run --example 2>&1)"

# Use tput command to manipulate the terminal
not_bold=$(tput sgr0)
bold=$(tput bold)
# Use ANSI escape characters to add colour to text
green='\033[0;32m'
red='\033[0;31m'
clear='\033[0m'

line_separator="==================="

# Exclude extra error text and get only example names, which start from 3rd line onwards
count=1
# IFS= means that string splitting occurs at newlines only
# read - reads a single line from the input
while IFS= read -r line
do
    if (( count > 2 )); then
        # Trim any whitespace before or after the string
        example_name=$(echo "$line" | xargs )
        example_label="\"$example_name\"";

        echo "${line_separator}"
        echo "Running ${example_label} example"

        cargo_run_cmd_segment="cargo run --example $example_name"

        # Handle example specific mandatory flags / options
        if [ "$example_name" == "falcon_discover_hosts" ]
        then
            cargo_run="$cargo_run_cmd_segment -- --sort hostname > /dev/null 2>&1"
        elif [ "$example_name" == "intel_indicators" ]
        then
            cargo_run="$cargo_run_cmd_segment -- --sort published_date.asc --filter deleted:false -q ps1 > /dev/null 2>&1"
        else
            cargo_run="$cargo_run_cmd_segment > /dev/null 2>&1"
        fi

        # Check command error code and exit if any error code
        if ! eval "$cargo_run"
        then
            echo -e "${example_label} ${red}${bold}failed${not_bold}${clear}"
            echo "${line_separator}"
            exit 1
        else
            echo -e "${example_label} ${green}${bold}passed${not_bold}${clear}"
        fi
    fi

    (( count++ ))
# The variable gets printed with a newline and fed into the while loop
done < <(printf '%s\n' "$cmd_output")
