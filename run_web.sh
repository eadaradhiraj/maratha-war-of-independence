#!/bin/bash
set -e

PORT=8080
echo "=========================================================="
echo " Starting Maratha: War of Independence (1700-1750)"
echo " Serving on http://localhost:$PORT (zero-cache mode)"
echo " Press Ctrl+C to terminate."
echo "=========================================================="

python3 server.py $PORT
