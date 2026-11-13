#!/usr/bin/env bash
# Downloads the General MIDI SoundFont the releases ship with, GeneralUser GS
# v2.0.3 by S. Christian Collins (redistributable, see its LICENSE.txt),
# pinned to one commit and checked against known SHA-256 sums.
#   tools/fetch-soundfont.sh <dir>   -> <dir>/GeneralUser-GS.sf2, <dir>/LICENSE.txt
# Then: SOUNDFONT=<dir>/GeneralUser-GS.sf2 SOUNDFONT_LICENSE=<dir>/LICENSE.txt tools/package-release.sh ...
set -euo pipefail
dir=${1:?usage: $0 <dir>}
commit=97049183643d5fc5a9322a69c5b09efb667c6c3a
base=https://raw.githubusercontent.com/mrbumpy409/GeneralUser-GS/$commit
mkdir -p "$dir"
curl -sSfL --retry 3 -o "$dir/GeneralUser-GS.sf2" "$base/GeneralUser-GS.sf2"
curl -sSfL --retry 3 -o "$dir/LICENSE.txt" "$base/documentation/LICENSE.txt"
# compare the hashes directly: GNU, BSD (macOS) and Perl checksum tools all
# print "<hash>  <file>", but their -c options differ
check() {
    local got
    got=$( (sha256sum "$dir/$1" 2>/dev/null || shasum -a 256 "$dir/$1") | cut -d' ' -f1)
    if [ "$got" != "$2" ]; then
        echo "$1: checksum mismatch (got $got)" >&2
        exit 1
    fi
    echo "$1: OK"
}
check GeneralUser-GS.sf2 9575028c7a1f589f5770fccc8cff2734566af40cd26ed836944e9a5152688cfe
check LICENSE.txt 7b32efefdf95ce38a043799f0659853ddc00fbaa14d8c50f0aca16b9b8b405be
