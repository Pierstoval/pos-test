#!/usr/bin/env bash

tag=$1

sed -i -E "s~^version *= *\"[^\"]*\"~version = \"$tag\"~gi" ./src-tauri/Cargo.toml

echo "v$tag-dev" > release-name
git add .
git cm -m "v$tag"
git tag -m "v$tag" "v$tag"
git push origin main --tags
gh release create "v$tag" --prerelease --title "v$tag" --notes ""
