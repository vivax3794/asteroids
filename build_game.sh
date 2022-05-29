rm -rf builds
mkdir builds
mkdir builds/linux

cross build --target x86_64-unknown-linux-gnu --release
cp -v  target/x86_64-unknown-linux-gnu/release/astroids builds/linux