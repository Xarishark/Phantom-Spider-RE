#!/usr/bin/env bash
# Cross-compiles the Windows build from Linux and packages it:
#   dist/PhantomSpiderRE/                              ready to run, or to load with
#                                                      RecompLauncher's "Add a build"
#   dist/PhantomSpiderRE-v<version>-Windows-x64.zip    for GitHub Releases
# Both hold PhantomSpider.exe, recomplauncher.json, How to play.txt and LICENSE,
# plus soundfont.sf2 if SOUNDFONT=/path/file.sf2 is set (GeneralUser GS may be
# redistributed; pass its license as SOUNDFONT_LICENSE=...).
# No original game files are included: players bring their own JAR.
# Needs: rustup target x86_64-pc-windows-gnu, mingw64-gcc, cmake, ninja, python3.
# SDL3 is built from source and linked statically, so the .exe has no DLLs.
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
cargo build --release --target x86_64-pc-windows-gnu
stage=dist/PhantomSpiderRE
mkdir -p "$stage"
cp target/x86_64-pc-windows-gnu/release/phantom-spider.exe "$stage/PhantomSpider.exe"
cp recomplauncher.json LICENSE "$stage/"
cat > "$stage/How to play.txt" <<TXT
Phantom Spider RE $version

This needs your own copy of the original game: Phantom_Spider_s40v2.jar
(Nokia Series 40, 128x128). It isn't included.

1. Put this folder somewhere you can write to (not Program Files).
2. Run PhantomSpider.exe and drop the JAR on the window, or press
   Fire / A to pick it. You can also put the JAR next to the exe.
3. Music needs a General MIDI SoundFont (.sf2) next to the exe.

Saves: save\   Settings: settings.toml   Logs: logs\
Controls and more: https://github.com/Xarishark/phantom-spider-re
TXT
if [ -n "${SOUNDFONT:-}" ]; then
    cp "$SOUNDFONT" "$stage/soundfont.sf2"
    [ -n "${SOUNDFONT_LICENSE:-}" ] && cp "$SOUNDFONT_LICENSE" "$stage/SOUNDFONT-LICENSE.txt"
fi
zip="dist/PhantomSpiderRE-v$version-Windows-x64.zip"
rm -f "$zip"
# only the release files: nothing a local run may have created (save/, data/, logs/, settings.toml)
python3 - "$stage" "$zip" <<'PY'
import os, sys, zipfile
stage, out = sys.argv[1], sys.argv[2]
keep = {"PhantomSpider.exe", "recomplauncher.json", "LICENSE", "How to play.txt",
        "soundfont.sf2", "SOUNDFONT-LICENSE.txt"}
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    for name in sorted(os.listdir(stage)):
        if name in keep:
            z.write(os.path.join(stage, name), name)
PY
echo "$stage/ and $zip:"
python3 -c "import zipfile,sys; [print(' ', i.filename, i.file_size) for i in zipfile.ZipFile(sys.argv[1]).infolist()]" "$zip"
