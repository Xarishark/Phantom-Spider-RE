#!/usr/bin/env bash
# Builds and packages a release for one system:
#   tools/package-release.sh [windows]  cross-compiles from Linux (MinGW) -> PhantomSpiderRE-v<ver>-Windows-x64.zip
#   tools/package-release.sh linux      native build                      -> PhantomSpiderRE-v<ver>-Linux-x64.tar.gz
#   tools/package-release.sh macos      native build on a Mac             -> PhantomSpiderRE-v<ver>-macOS-<arch>.zip
# dist/PhantomSpiderRE/ is the same folder unpacked, ready to run or to load
# with RecompLauncher's "Add a build". Each holds the program,
# recomplauncher.json (top level, for RecompLauncher), How to play.txt and
# LICENSE, plus soundfont.sf2 if SOUNDFONT=/path/file.sf2 is set
# (GeneralUser GS may be redistributed; pass its license as SOUNDFONT_LICENSE=...).
# SDL3 is built from source and linked statically, so nothing else needs
# installing. No original game files are included: players bring their own JAR.
# Needs: cmake, python3; for windows also rustup target x86_64-pc-windows-gnu
# and mingw64-gcc; for linux the X11/Wayland/audio development headers (see
# .github/workflows/build.yml).
set -euo pipefail
cd "$(dirname "$0")/.."
os=${1:-windows}
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
target=${CARGO_TARGET_DIR:-target}
stage=dist/PhantomSpiderRE
# never wipe the stage: it may be a played folder with save/, data/ and settings.toml
rm -f "$stage/PhantomSpider.exe" "$stage/phantom-spider"
mkdir -p "$stage"
case "$os" in
    windows)
        cargo build --release --locked --target x86_64-pc-windows-gnu
        cp "$target/x86_64-pc-windows-gnu/release/phantom-spider.exe" "$stage/PhantomSpider.exe"
        program=PhantomSpider.exe
        archive="dist/PhantomSpiderRE-v$version-Windows-x64.zip"
        ;;
    linux | macos)
        cargo build --release --locked
        cp "$target/release/phantom-spider" "$stage/phantom-spider"
        program=phantom-spider
        if [ "$os" = linux ]; then
            archive="dist/PhantomSpiderRE-v$version-Linux-x64.tar.gz"
        else
            arch=$(uname -m)
            [ "$arch" = x86_64 ] && arch=x64
            archive="dist/PhantomSpiderRE-v$version-macOS-$arch.zip"
        fi
        ;;
    *)
        echo "usage: $0 windows|linux|macos" >&2
        exit 2
        ;;
esac
cp recomplauncher.json LICENSE "$stage/"
mac_note=""
[ "$os" = macos ] && mac_note="
The build isn't signed: the first time, right-click phantom-spider and
choose Open, or run: xattr -d com.apple.quarantine phantom-spider
"
cat > "$stage/How to play.txt" <<TXT
Phantom Spider RE $version

This needs your own copy of the original game: Phantom_Spider_s40v2.jar
(Nokia Series 40, 128x128). It isn't included.

1. Put this folder somewhere you can write to.
2. Run $program and drop the JAR on the window, or press
   Enter to pick it. You can also put the JAR next to the program.
3. Music needs a General MIDI SoundFont (.sf2) next to the program.
$mac_note
Saves: save/   Settings: settings.toml   Logs: logs/
Controls and more: https://github.com/Xarishark/phantom-spider-re
TXT
if [ -n "${SOUNDFONT:-}" ]; then
    cp "$SOUNDFONT" "$stage/soundfont.sf2"
    [ -n "${SOUNDFONT_LICENSE:-}" ] && cp "$SOUNDFONT_LICENSE" "$stage/SOUNDFONT-LICENSE.txt"
fi
rm -f "$archive"
# only the release files: nothing a local run may have created (save/, data/, logs/, settings.toml)
python3 - "$stage" "$archive" "$program" <<'PY'
import os, sys, tarfile, zipfile
stage, out, program = sys.argv[1:4]
keep = [program, "recomplauncher.json", "LICENSE", "How to play.txt", "soundfont.sf2", "SOUNDFONT-LICENSE.txt"]
keep = [n for n in keep if os.path.exists(os.path.join(stage, n))]
if out.endswith(".tar.gz"):
    with tarfile.open(out, "w:gz") as t:
        for name in keep:
            t.add(os.path.join(stage, name), name)
else:
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for name in keep:
            path = os.path.join(stage, name)
            info = zipfile.ZipInfo.from_file(path, name)  # keeps the executable bit
            info.compress_type = zipfile.ZIP_DEFLATED
            with open(path, "rb") as f:
                z.writestr(info, f.read())
print(f"{stage}/ and {out}:")
for name in keep:
    print(" ", name, os.path.getsize(os.path.join(stage, name)))
PY
