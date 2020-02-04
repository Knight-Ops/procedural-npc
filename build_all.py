import sys
import os
import shutil

if os.name == 'nt':
    sys.path.append(".\\ui")
    sys.path.append(".\\server")
else:
    sys.path.append("ui/")
    sys.path.append("server/")
import build_ui
import build_server


def build_all():
    if os.name == 'nt':
        os.chdir(".\\ui")
    else:
        os.chdir("ui/")

    build_ui.build_and_move_ui()

    if os.name == 'nt':
        os.chdir("..\\server")
    else:
        os.chdir("../server")
    build_server.build_server()

    print("Copying files to server...")
    if os.name == 'nt':
        shutil.copy("..\\ui\\static\\ui.js", "static\\ui.js")
        shutil.copy("..\\ui\\static\\ui.wasm", "static\\ui.wasm")
        shutil.copy("..\\ui\\static\\style.css", "static\\style.css")
    elif os.name == 'posix':
        shutil.copy("../ui/static/ui.js", "static/ui.js")
        shutil.copy("../ui/static/ui.wasm", "static/ui.wasm")
        shutil.copy("../ui/static/style.css", "static/style.css")
    print("Copying files to server complete!")

if __name__ == "__main__":
    build_all()