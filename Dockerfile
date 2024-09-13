FROM rust:slim-bullseye

RUN cargo install wasm-pack
RUN cargo install minifier

RUN apt update
RUN apt install curl -y
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz
RUN gunzip elm.gz
RUN chmod +x elm
RUN mv elm /usr/local/bin/
RUN elm --help

WORKDIR /usr/src/app

COPY . .

RUN cd elm && elm make src/Main.elm --output=elm.js --optimize && cd ..
RUN cd rust && wasm-pack build --target no-modules --no-typescript --release && cd ..

RUN mkdir build
 
RUN cp rust/pkg/rust_bg.wasm build/rust_bg.wasm
RUN cp web/index.html build/index.html
 
RUN minifier -o build/style.css web/style.css
RUN minifier -o build/rust.js rust/pkg/rust.js
RUN minifier -o build/script.js web/script.js

CMD ["bash"]