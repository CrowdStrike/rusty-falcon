#!/bin/bash
#
# Perform testing/validation of Rusty Falcon examples

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
passed=0

# Initial message when the script just starts running
echo ""
echo "Starting to test examples"
echo ""

# Capture the first argument
EXAMPLE=$1

#######################################
# Handle individual example cases and
# build the cargo command to execute.
#
# Arguments:
#   Example name.
#
# Returns:
#   Complete cargo command to run.
#######################################
handle_example () {
    local example=$1
    local cargo_run_cmd_segment="cargo run --example $example"

    if [[ "${example}" == "falcon_discover_hosts" ]]; then
        cargo_run="$cargo_run_cmd_segment -- --sort hostname 2>&1"
    elif [[ "${example}" == "falcon_supported_kernels" ]]; then
        cargo_run="$cargo_run_cmd_segment -- --distro=oracle6 --arch=aarch64"
    elif [[ "${example}" == "intel_indicators" ]]; then
        cargo_run="$cargo_run_cmd_segment -- --sort published_date.asc --filter deleted:false -q ps1 2>&1"
    else
        cargo_run="$cargo_run_cmd_segment 2>&1"
    fi

    echo "$cargo_run"
}

# Run a single example if passed in or run all examples otherwise
if [ -n "$EXAMPLE" ]; then
    echo "Testing ${EXAMPLE}"

    cargo_run=$(handle_example "$EXAMPLE")

    # Check command error code and exit if any error code
    if ! command=$(eval "$cargo_run"); then
        echo "$command"
        echo -e "${example_label} ${red}${bold}failed${not_bold}${clear}"
    else
        echo "$command"
        echo -e "${example_label} ${green}${bold}passed${not_bold}${clear}"
    fi
else
    # IFS= means that string splitting occurs at newlines only
    # read - reads a single line from the input 
    # (in this case, input being the error that arises when running 
    # "cargo run --example" without specifying the example to run)
    while IFS= read -r line; do
        # First two lines of stderr are just error text and are irrelevant to the logic below
        if (( count > 2 )); then
            # Trim any whitespace before or after the string
            example_name=$(echo "$line" | xargs )
            example_label="\"$example_name\"";

            echo "${line_separator}"
            echo "Testing ${example_label}"

            cargo_run=$(handle_example "$example_label")

            # Check command error code and exit if any error code
            if ! command=$(eval "$cargo_run"); then
                echo "$command"
                echo -e "${example_label} ${red}${bold}failed${not_bold}${clear}"
            else
                echo -e "${example_label} ${green}${bold}passed${not_bold}${clear}"
                (( passed++ ))
            fi
        fi

        (( count++ ))
    # The variable gets printed with a newline and fed into the while loop
    done < <(printf '%s\n' "$cmd_output")

    # Print last line separator and tests stats
    echo "${line_separator}"
    total=$(( count - 3 ))
    echo "${passed} out of ${total} passed"
fi
