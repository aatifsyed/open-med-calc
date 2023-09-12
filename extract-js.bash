#!/usr/bin/env bash
set -euxo pipefail

folder=js
rm -rf "$folder"
mkdir --parents "$folder"
echo '*' > "$folder/.gitignore"

for path in scraped/calc/*.json; do
    slug=$(basename --suffix .json "$path")
    jq -r '.props.pageProps.calc.equation_logic_text' "$path" \
        > "$folder/$slug.js" \
        &
done

wait
