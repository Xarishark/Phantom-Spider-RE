#!/usr/bin/env bash
# Runs every headless test script against a release build.
# Screenshots go to target/test-shots/<script>/, saves to a temp folder
# (never to the real save folder next to the binary).
set -uo pipefail
cd "$(dirname "$0")/.."
cargo build --release || exit 1
bin=target/release/phantom-spider
[ -x "$bin" ] || bin=target/release/phantom-spider.exe
fail=0
for script in tests/scripts/*.txt; do
    name=$(basename "$script" .txt)
    out=target/test-shots/$name
    data=$(mktemp -d)
    log=$(mktemp)
    rm -rf "$out"
    echo "== $name"
    PHANTOM_SPIDER_DATA="$data" "$bin" --headless --script "$script" --out "$out" >"$log" 2>&1
    status=$?
    grep -E '^(pos|stat)' "$log" | sed 's/^/   /'
    if [ "$status" -ne 0 ]; then
        echo "   FAILED (exit $status)"
        sed 's/^/   | /' "$log" | tail -5
        fail=1
    else
        echo "   ok, $(ls "$out" 2>/dev/null | wc -l) screenshots in $out"
    fi
    rm -rf "$data" "$log"
done
exit $fail
