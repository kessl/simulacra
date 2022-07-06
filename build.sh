#!/bin/sh

set -ex

wasm-pack build --debug --target web
python3 -m http.server
