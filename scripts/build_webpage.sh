rm -rfv builds/web
rm -fv builds/web.zip
mkdir builds/web

echo "BUILDING WEB"
cargo build --target wasm32-unknown-unknown --release
echo "GENERATING WEB BINDINGS"
wasm-bindgen --no-typescript --out-dir ./builds/web --target web ./target/wasm32-unknown-unknown/release/astroids.wasm
echo "OPTIMIZING WASM BUILD"
wasm-opt -O3 -o ./builds/web/astroids_bg.wasm ./builds/web/astroids_bg.wasm

echo "CREATING WEBAPGE"
cp -v scripts/index.html ./builds/web/
cp -rv assets builds/web
cd builds/web
zip -r ../web.zip *
cd ../..
rm -rfv builds/web