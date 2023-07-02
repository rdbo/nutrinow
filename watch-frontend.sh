#!/bin/sh
# Builds the frontend whenever a change occurs in the frontend directory
# Requires: entr

cd frontend
npm run watch
