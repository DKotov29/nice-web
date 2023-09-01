#!/bin/sh

# Change to the web-client directory
cd web-client

# Build the web-client using the trunk command
trunk build

# Move the built files from web-client/dist to nice-web-backend/static
mv dist/* ../nice-web/static

# Change to the nice-web-backend directory
cd ../nice-web

# Build the nice-web-backend server
cargo run --bin nice-web
