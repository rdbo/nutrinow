#!/bin/sh
# Builds the frontend whenever a change occurs in the frontend directory
# Requires: entr

while :; do
    find frontend -not \( -path frontend/node_modules -prune \) -not \( -path frontend/dist -prune \) | entr -d sh ./build-frontend.sh
done
