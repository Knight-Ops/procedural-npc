import subprocess
import shutil
import os

def build_server():
    print("Building with cargo...")
    subprocess.run(["cargo", "+nightly", "build", "--release"])

    if os.name == "posix":
        subprocess.run(["cargo", "+nightly", "build", "--target", "aarch64-unknown-linux-gnu", "--release"])

if __name__ == "__main__":
    build_server()