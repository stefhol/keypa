#!/bin/bash
# Start the process
cd /usr/local/app/
chmod +x ./api
nginx -g "daemon off;" &
./api &

# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?