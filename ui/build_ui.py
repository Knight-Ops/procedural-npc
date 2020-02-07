import subprocess
import shutil
import os

def build_and_move_ui():
    print("Building with cargo web...")
    subprocess.run(["cargo", "web", "build", "--release"])

    print("Copying built files to static directory...")
    if os.name == 'nt':
        shutil.copy("target\\wasm32-unknown-unknown\\release\\ui.js", "static\\ui.js")
        shutil.copy("target\\wasm32-unknown-unknown\\release\\ui.wasm", "static\\ui.wasm")
    elif os.name == 'posix':
        shutil.copy("target/wasm32-unknown-unknown/release/ui.js", "static/ui.js")
        shutil.copy("target/wasm32-unknown-unknown/release/ui.wasm", "static/ui.wasm")
    print("Copying files completed!")

if __name__ == "__main__":
    build_and_move_ui()