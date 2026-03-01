
$Script:tag=$args[0]

echo "ok == $tag =="

git add .

git cm -m-

git tag -m $tag $tag

git push origin main --tags

gh release create $tag --prerelease --title=$tag
