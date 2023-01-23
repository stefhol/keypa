#!/bin/bash
# Start the process
nginx &
sh /usr/local/app/api &
# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?