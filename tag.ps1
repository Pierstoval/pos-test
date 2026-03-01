
$Script:tag=$args[0]

sed -i .\src-tauri\Cargo.toml -E "s/version = `"[0-9\.]+`"/version = `"$tag`"/g"

git add .

git cm -m-

git tag -m v$tag v$tag

git push origin main --tags

gh release create v$tag --prerelease --title v$tag --notes ""
