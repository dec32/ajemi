setup:
    rustup target add x86_64-pc-windows-msvc
    rustup target add i686-pc-windows-msvc
    wget https://raw.githubusercontent.com/kreativekorp/sitelen-seli-kiwen/refs/heads/main/sitelenselikiwenjuniko.ttf -OutFile .\res\sitelenselikiwenjuniko.ttf
fmt:
    cargo +nightly fmt --all
build *args: 
    regsvr32 /s /u  .\target\debug\ajemi.dll
    regsvr32 /s /u  .\target\i686-pc-windows-msvc\debug\ajemi.dll
    cargo build {{args}}
    cargo build --target=i686-pc-windows-msvc {{args}}
    regsvr32 /s .\target\debug\ajemi.dll
    regsvr32 /s .\target\i686-pc-windows-msvc\debug\ajemi.dll
unreg: 
    regsvr32 /s /u  .\target\debug\ajemi.dll
    regsvr32 /s /u  .\target\i686-pc-windows-msvc\debug\ajemi.dll
pack:
    cargo build --release
    cargo build --release --target=i686-pc-windows-msvc
    iscc .\installer.iss
release:
    just pack
    git push --delete origin nightly
    gh release create nightly ".\target\release\ajemi-installer_x64.exe" -t "Nightly Build" -n "Nightly Build"   
