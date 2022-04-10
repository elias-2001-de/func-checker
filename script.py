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
        if sys.argv[1] == "release":
            is_release = True
        elif sys.argv[1] == "test":
            os.system("cargo test && wasm-pack test --node")
            return

    if not os.path.exists("build"):
        os.makedirs("build")
    else:
        for file in listdir("build"):
            os.remove("build/"+file)

    os.system("trunk build " +
              ("--release" if is_release else ""))

    for file in listdir("dist"):
        if ".js" in file:
            minifie("dist/"+file, "build/"+file)
        else:
            shutil.copy2("dist/"+file, "build/"+file)
        
    with open("build/index.html", "r+") as file:
        value = file.read()
        file.seek(0)
        file.write(value.replace('/index', './index'))


if __name__ == "__main__":
    main()
