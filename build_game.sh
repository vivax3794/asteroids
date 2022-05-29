rm -rf builds
mkdir builds
mkdir builds/linux

cargo build -r
cp target/release/astroids builds/linux