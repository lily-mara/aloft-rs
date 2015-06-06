#!/bin/bash

# script curtesy of http://www.steveklabnik.com/automatically_update_github_pages_with_travis_example/

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cargo doc
cd target/doc/aloft

git init
git config user.name "Nate Mara"
git config user.email "natemara@gmail.com"

git remote add upstream "https://$GH_TOKEN@github.com/natemara/aloft-rs.git"
git fetch upstream
git reset upstream/gh-pages

# echo "rustbyexample.com" > CNAME

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
