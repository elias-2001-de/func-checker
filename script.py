import sys
import os
import shutil
from jsmin import jsmin
from os import listdir

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

def makedir(path):
    if not os.path.exists(path):
        os.makedirs(path)

def main():
    global is_release

    if len(sys.argv) == 2:
        # set release
        if sys.argv[1] == "release":
            is_release = True
        # test then exit
        elif sys.argv[1] == "test":
            # test 
            os.system("cargo test && wasm-pack test --node")
            return

    # create build dir
    if not os.path.exists("build"):
        os.makedirs("build")
    else:
        for file in listdir("build"):
            os.remove("build/"+file)

    # compile to wasm
    os.system("trunk build " +
              ("--release" if is_release else ""))

    for file in listdir("dist"):
        if ".js" in file:
            minifie("dist/"+file, "build/"+file)
        else:
            shutil.copy2("dist/"+file, "build/"+file)


if __name__ == "__main__":
    main()
