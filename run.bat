@echo off

REM Change to the web-client directory
cd web-client

REM Build the web-client using the trunk command
trunk build

REM Move the built files from web-client/dist to nice-web/static
move dist\* ..\nice-web\static

REM Change to the nice-web directory
cd ..\nice-web

REM Build the nice-web server
cargo run --bin nice-web
REM or build