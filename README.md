# Amiga BBS Terminal Emulator

A WebAssembly-based Amiga terminal emulator for connecting to BBS systems, specifically designed for AmiExpress.

## Features

- Authentic Amiga terminal display (640x256 PAL resolution)
- Topaz font rendering  2
- ANSI color support
- WebSocket-based BBS connectivity

## Development Setup

1. Install prerequisites:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install wasm-pack
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/amiga-bbs-wasm.git
   cd amiga-bbs-wasm
   ```

3. Build the project:
   ```bash
   ./build.sh
   ```

4. Open http://localhost:8000 in your browser

## Project Structure

- `src/lib.rs` - Main terminal emulator implementation
- `src/bbs_connection.rs` - WebSocket-based BBS connectivity
- `src/keyboard.rs` - Keyboard input handling
- `index.html` - Web interface
- `assets/` - Fonts and other resources

## License

MIT License (or choose your preferred license) 