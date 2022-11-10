import sys
import os
import shutil
from jsmin import jsmin

is_release = False


def minifie(src, dest):
    if is_release:
        with open(src) as input:
            minified = jsmin(input.read())
            output = open(dest, 'w+')
            output.write(minified)
            output.close()
    else:
        shutil.copy2(src, dest)


def main():
    global is_release

    if len(sys.argv) == 2:
        # set release
        if sys.argv[1] == "release":
            is_release = True
        # test then exit
        elif sys.argv[1] == "test":
            # test rust
            os.system("cd rust && cargo test && wasm-pack test --node")
            # test elm
            # !! not yet avalibal for elm version 0.19      os.system("cd elm && ")
            return

    # create build dir
    if not os.path.exists("build"):
        os.makedirs("build")

    # elm
    os.system("cd elm && elm make src/Main.elm --output=elm.js " +
              "--optimize" if is_release else "")
    minifie("elm/elm.js", "build/elm.js")

    # rust
    # compile to wasm
    os.system("cd rust && wasm-pack build --target no-modules --no-typescript " +
              ("--release" if is_release else ""))
    # copy js to build
    shutil.copy2("rust/pkg/rust_bg.wasm", "build/rust_bg.wasm")
    minifie("rust/pkg/rust.js", "build/rust.js")

    # js
    minifie("web/script.js", "build/script.js")

    # html 
    shutil.copy2("web/index.html", "build/index.html")

    # css
    shutil.copy2("web/style.css", "build/style.css")



if __name__ == "__main__":
    main()