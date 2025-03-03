param (
    [switch]$Release
)

if ($Release) {
    cargo build --release
    cargo build --release --target=i686-pc-windows-msvc
    iscc .\installer.iss
} else {
    regsvr32 /s /u  .\target\debug\ajemi.dll
    regsvr32 /s /u  .\target\i686-pc-windows-msvc\debug\ajemi.dll
    cargo build
    cargo build --target=i686-pc-windows-msvc
    regsvr32 /s .\target\debug\ajemi.dll
    regsvr32 /s .\target\i686-pc-windows-msvc\debug\ajemi.dll
}


