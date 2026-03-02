
$Script:tag=$args[0]

sed -i -E "s~^version *= *`"[^`"]*`"~version = `"$tag`"~gi" .\src-tauri\Cargo.toml

Write-Output "v$tag-dev" > release-name
git add .
git cm -m "v$tag" --allow-empty
git tag -m "v$tag" "v$tag"
git push origin main --tags
gh release create "v$tag" --prerelease --title "v$tag" --notes ""
