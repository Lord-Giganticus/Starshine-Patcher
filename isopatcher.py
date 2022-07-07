from urllib.request import urlopen, Request
from zipfile import ZipFile
from pathlib import Path
from platform import system
from enum import Enum
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
    
    def executeandcaptrue(self, args: str):
        proc = subprocess.run(f'{self.program} {args}', stdout=subprocess.PIPE, shell=True)
        return proc.stdout.decode('utf-8')
        
class Region(Enum):
    USA = 'E'
    PAL = 'P'
    JAP = 'J'
    KOR = 'K'
    TWN = 'W'

SYATI = URLDependency('https://github.com/SunakazeKun/Syati/archive/refs/heads/main.zip')
SYATISETUP = URLDependency('https://github.com/Lord-Giganticus/SyatiSetup/releases/download/Auto/syatisetup.exe')
WIT = URLDependency('https://cdn.discordapp.com/attachments/886616711925751829/993881870666305578/wit.zip')
STARSHINE = URLDependency('https://github.com/Lord-Giganticus/Lord-Giganticus.github.io/releases/download/auto/Super.Mario.Starshine.Demo.v1.4.zip')
PATCHES = Path('patches.xml').absolute()
HOME_DIR = Path(__file__).parent.absolute()
SYSTEM = system().lower()

if __name__ == '__main__':
    if SYSTEM != 'windows':
        raise Exception("Non-Windows systems are not supported ATM.")
    
    name = WIT.download()

    with ZipFile(name, "r") as r:
        r.extractall()

    WIT = FileDependency(Path("wit\\wit.exe"))

    os.remove(name)

    print("Extracting SMG2...")

    WIT.execute(f'extract -s ../ -1 -n SB4.01 . smg2.d --psel=DATA -ovv')

    reg = WIT.executeandcaptrue('ID6 -s ../ -1 -n SB4.01 .')[3]

    region = f'{Region(reg)}'

    region = region[region.index('.') + 1:]

    print("Downloading Starshine...")

    name = STARSHINE.download()

    print("Extracting Starshine...")

    with ZipFile(name, "r") as r:
        r.extractall()

    os.remove(name)

    print("Copying Starshine files to extracted SMG2...")

    orig = Path('smg2.d\\files').absolute()
    
    starshine = Path('Super Mario Starshine Demo v1.4\\Super Mario Starshine').absolute()

    for dir in starshine.glob('*'):
        if dir.name == "AudioRes" or dir.name == "StageData":
            for idir in dir.glob('*'):
                for file in idir.glob('*.*'):
                    os.makedirs(f'{orig}\\{dir.name}\\{idir.name}', exist_ok=True)
                    shutil.copy(file, f'{orig}\\{dir.name}\\{idir.name}\\{file.name}')
        elif dir.name == "LocalizeData":
            for ldir in dir.glob('*'):
                    for idir in ldir.glob('*'):
                        os.makedirs(f'{orig}\\{dir.name}\\{ldir.name}\\{idir.name}', exist_ok=True)
                        for file in idir.glob('*'):
                            shutil.copy(file, f'{orig}\\{dir.name}\\{ldir.name}\\{idir.name}\\{file.name}')
        elif dir.is_dir():
            os.makedirs(f'{orig}\\{dir.name}', exist_ok=True)
            for file in dir.glob('*'):
                shutil.copy(file, f'{orig}\\{dir.name}\\{file.name}')
        elif dir.is_file():
            shutil.copy(dir, f'{orig}\\{dir.name}')

    print("Downloading Syati...")

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

    print("Patching SMG2's main.dol...")        

    filepath = Path(f'{HOME_DIR}\\smg2.d\\sys\\main.dol').absolute()

    shutil.copy(filepath, f'dols\\{region}.dol')

    subprocess.run(f'python buildloader.py {region}', shell=True)

    WIT.execute(f'dolpatch loader\\{region}.dol xml={PATCHES}')

    shutil.copyfile(f'loader\\{region}.dol', filepath)

    os.chdir(HOME_DIR)

    print("Rebuilding SMG2...")

    WIT.execute('copy smg2.d "Super Mario Starshine.wbfs" -ovv --id=....01 --name="Super Mario Starshine"')

    shutil.rmtree('smg2.d')

    shutil.rmtree('Super Mario Starshine Demo v1.4')

    shutil.rmtree('wit')

    shutil.rmtree('Syati-main')

    print("Completed!")