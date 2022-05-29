rm -rfv builds/windows
rm -fv builds/windows.zip
mkdir builds/windows

echo "BUILDING WINDOWS"
cross build --target x86_64-pc-windows-gnu --release
cp -v  target/x86_64-pc-windows-gnu/release/astroids.exe builds/windows
cp -vr assets builds/windows

echo "PACKACING WINDOWS"
cd builds/windows
zip -r ../windows.zip *
cd ../..