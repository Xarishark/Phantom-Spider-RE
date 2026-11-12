#!/usr/bin/env bash
# Cross-compiles the Windows .exe from Linux and assembles dist/PhantomSpider/.
# Needs: rustup target x86_64-pc-windows-gnu, mingw64-gcc, cmake, ninja.
# SDL3 is built from source and linked statically, so the .exe has no DLLs.
# Set SOUNDFONT=/path/to/file.sf2 to bundle a General MIDI SoundFont.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release --target x86_64-pc-windows-gnu
mkdir -p dist/PhantomSpider
cp target/x86_64-pc-windows-gnu/release/phantom-spider.exe dist/PhantomSpider/PhantomSpider.exe
if [ -n "${SOUNDFONT:-}" ]; then
    cp "$SOUNDFONT" dist/PhantomSpider/soundfont.sf2
fi
echo "dist/PhantomSpider:"
ls -la dist/PhantomSpider
