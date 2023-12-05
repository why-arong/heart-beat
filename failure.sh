#!/bin/bash
pipe_path=$1

# Read from the named pipe
read exit_code unix_time < "$pipe_path"

# Set environment variables
export HEAT_FAIL_CODE=$exit_code
export HEAT_FAIL_TIME=$unix_time
# export HEAT_FAIL_INTERVAL=$
# export HEAT_FAIL_PID=$

# Optional: Print the variables for confirmation
echo "HEAT_FAIL_CODE set to $HEAT_FAIL_CODE"
echo "HEAT_FAIL_TIME set to $HEAT_FAIL_TIME"