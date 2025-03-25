FROM rust:1.85-slim-bullseye

RUN cargo install wasm-pack

WORKDIR /app

COPY . .

RUN wasm-pack build --target no-modules --no-typescript --release 
RUN cp pkg/func_checker_bg.wasm web
RUN cp pkg/func_checker.js web/wasm_loader.js 

CMD ["bash"]