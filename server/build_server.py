import subprocess
import shutil
import os

def build_server():
    print("Building with cargo...")
    subprocess.run(["cargo", "+nightly", "build", "--release"])

if __name__ == "__main__":
    build_server()