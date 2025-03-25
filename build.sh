wasm-pack build --target no-modules --no-typescript --release 
cp pkg/func_checker_bg.wasm web
cp pkg/func_checker.js web/wasm_loader.js 