#!/bin/bash
# pipe_path=$1

# # Read from the named pipe
# read exit_code unix_time interval fail_pid < "$pipe_path"

# # Set environment variables
# export HEAT_FAIL_CODE=$exit_code
# export HEAT_FAIL_TIME=$unix_time
# export HEAT_FAIL_INTERVAL=$interval
# export HEAT_FAIL_PID=$fail_pid
# execve  (g환경변수전달)
# Optional: Print the variables for confirmation
echo "HEAT_FAIL_CODE set to $HEAT_FAIL_CODE"
echo "HEAT_FAIL_TIME set to $HEAT_FAIL_TIME"
echo "HEAT_FAIL_INTERVAL set to $HEAT_FAIL_INTERVAL"
echo "HEAT_FAIL_PID set to $HEAT_FAIL_PID"
