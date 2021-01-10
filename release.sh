cargo build --release
mkdir -p release/iz-cpm-for-linux
cp README.md download.sh target/release/iz-cpm release/iz-cpm-for-linux/
zip -r release/iz-cpm-for-linux.zip release/iz-cpm-for-linux

cargo build --release --target x86_64-pc-windows-gnu
mkdir -p release/iz-cpm-for-windows
cp README.md download.bat target/x86_64-pc-windows-gnu/release/iz-cpm.exe release/iz-cpm-for-windows/
zip -r release/iz-cpm-for-windows.zip release/iz-cpm-for-windows

