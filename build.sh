cd elm && elm make src/Main.elm --output=elm.js --optimize && cd ..
cd rust && wasm-pack build --target no-modules --no-typescript --release && cd ..

mkdir build

cp rust/pkg/rust_bg.wasm build/rust_bg.wasm
cp web/index.html build/index.html

minifier -o build/style.css web/style.css
minifier -o build/rust.js rust/pkg/rust.js
minifier -o build/script.js web/script.js