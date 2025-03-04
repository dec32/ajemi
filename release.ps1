.\build -Release
git push --delete origin nightly
gh release create nightly ".\target\release\ajemi-installer_x64.exe" -t "Nightly Build" -n "Nightly Build"