from pathlib import Path
from zipfile import ZipFile
import subprocess
import os

parent = Path(__file__).absolute().parent.absolute()

os.chdir(parent)

with ZipFile(f"{parent}\\Starshine-Patcher.zip", "w") as w:
    w.write("LICENSE")
    w.write("README.md")
    w.write("python\\isopatcher.py")
    os.chdir("rust")
    subprocess.run("cargo build --release")
    os.chdir(parent)
    w.write("rust\\target\\release\\starshine-patcher.exe", "rust\\normal\\starshine-patcher.exe")
    os.chdir("rust")
    subprocess.run("cargo build --release --features pyo3")
    os.chdir(parent)
    w.write("rust\\target\\release\\starshine-patcher.exe", "rust\\pyo3\\starshine-patcher.exe")