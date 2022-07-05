from urllib.request import urlopen, Request
from zipfile import ZipFile
from pathlib import Path
from platform import system
from tkinter import filedialog
from sys import argv
import os
import subprocess
import shutil

class URLDependency:
    def __init__(self, url: str):
        self.url = url
    
    def download(self, fullname: str = None):
        url = self.url
        if "github.com" in url and fullname == None and '/refs/heads' in url:
            idx = url.index('.com') + 5
            idx = url.index('/', idx) + 1
            name = url[idx:url.index('/', idx)]
            idx = url.rindex('/') + 1
            fullname = f'{name}-{url[idx:]}'
        elif fullname == None:
            idx = url.rindex('/') + 1
            fullname = url[idx:]
        # Allows urlopen to use cdn.discordapp.com
        req = Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urlopen(req) as req, open(fullname, "wb") as w:
            data = req.read()
            w.write(data)
        return fullname

class FileDependency:
    def __init__(self, program: Path):
        if program.exists() != True:
            raise FileNotFoundError(f"{program.name} not found.")
        self.program = program.absolute()

    def execute(self, args: str):
        subprocess.run(f'{self.program} {args}')

SYATI = URLDependency('https://github.com/SunakazeKun/Syati/archive/refs/heads/main.zip')
SYATISETUP = URLDependency('https://github.com/Lord-Giganticus/SyatiSetup/releases/download/Auto/syatisetup.exe')
WIT = URLDependency('https://cdn.discordapp.com/attachments/886616711925751829/993881870666305578/wit.zip')
PATCHES = Path('patches.xml').absolute()
HOME_DIR = Path(__file__).parent.absolute()
SYSTEM = system().lower()

if __name__ == '__main__':
    if len(argv) < 2:
        raise Exception("Did not specify a target region.")
    region = argv[1]
    if SYSTEM != 'windows':
        print(SYSTEM)
        raise Exception("Non-Windows systems are not supported ATM.")

    name = WIT.download()

    with ZipFile(name, "r") as r:
        r.extractall()

    WIT = FileDependency(Path("wit\\wit.exe"))

    os.remove(name)

    name = SYATI.download()

    with ZipFile(name, "r") as r:
        r.extractall()

    os.remove(name)

    name = name[:name.rindex('.')]

    os.chdir(name)

    if os.path.exists("deps") != True:
        os.mkdir('deps')

    if os.path.exists('dols') != True:
        os.mkdir('dols')

    os.chdir('deps')

    SYATISETUP.download()

    subprocess.run("syatisetup.exe", shell=True)

    os.chdir('../')
    filetypes = (('Dol files', '*.dol'), ('All files', '*.*'))
    file = filedialog.askopenfilename(defaultextension='.dol', filetypes=filetypes, 
    title="Provide your SMG2 main.dol file.")

    filepath = Path(file).absolute()

    shutil.copyfile(filepath, f'dols\\{region}.dol')

    subprocess.run(f'python buildloader.py {region}', shell=True)

    WIT.execute(f'dolpatch loader\\{region}.dol xml={PATCHES}')

    shutil.copyfile(f'loader\\{region}.dol', filepath)

    print("Finished!")