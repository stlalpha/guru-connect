#!/bin/bash
wasm-pack build --target web
python3 -m http.server # Or any other local server 