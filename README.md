## Install

### docker

```
docker build -t func-checker-build . 
docker create --name extract-container func-checker-build
docker cp extract-container:/usr/src/app/build/ .
docker rm extract-container
```

### local


you need [elm](https://guide.elm-lang.org/install/elm.html) and [rust]() installed 

```
cargo install wasm-pack minifier
./build.sh
```