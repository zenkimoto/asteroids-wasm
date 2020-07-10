#!/bin/bash

cargo web deploy --features "quicksilver/stdweb getrandom/stdweb"

# Install basic http server to use
# https://github.com/brson/basic-http-server
basic-http-server target/deploy
