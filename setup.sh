#!/bin/bash
cargo new amiga-bbs-wasm --lib
cd amiga-bbs-wasm

# Add dependencies with proper features
cargo add wasm-bindgen
cargo add web-sys --features="Window,Document,Element,HtmlCanvasElement,CanvasRenderingContext2d,WebSocket,MessageEvent,KeyboardEvent,BinaryType"
cargo add console_error_panic_hook
cargo add futures wasm-bindgen-futures
cargo add serde --features="derive"
cargo add serde_json
cargo add js-sys 

mkdir -p assets
curl -L https://github.com/rewtnull/amigafonts/raw/master/TopazPlus_a1200.ttf -o assets/TopazPlus_a1200.ttf