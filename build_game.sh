rm -rf builds
mkdir builds
mkdir builds/linux
mkdir builds/windows

cross build --target x86_64-unknown-linux-gnu --release
cross build --target x86_64-pc-windows-gnu --release

cp -v  target/x86_64-unknown-linux-gnu/release/astroids builds/linux
cp -v  target/x86_64-pc-windows-gnu/release/astroids.exe builds/windows
