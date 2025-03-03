rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc
wget https://www.kreativekorp.com/swdownload/fonts/xlang/sitelenselikiwen.zip -OutFile .\tmp\sitelenselikiwen.zip
Expand-Archive .\tmp\sitelenselikiwen.zip -DestinationPath .\tmp
Move-Item .\tmp\sitelenselikiwenjuniko.ttf .\res\sitelenselikiwenjuniko.ttf -Force
Remove-Item .\tmp -Recurse
