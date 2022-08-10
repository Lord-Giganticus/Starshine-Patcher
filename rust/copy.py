from pathlib import Path
import shutil
import os

orig = Path('smg2.d\\files').absolute()
    
starshine = Path('Super Mario Starshine Demo v1.4\\Super Mario Starshine').absolute()

for dir in starshine.glob('*'):
    if dir.name == "AudioRes" or dir.name == "StageData":
        for idir in dir.glob('*'):
            for file in idir.glob('*.*'):
                os.makedirs(f'{orig}\\{dir.name}\\{idir.name}', exist_ok=True)
                dest = f'{orig}\\{dir.name}\\{idir.name}\\{file.name}'
                with open(file, "rb") as r, open(dest, "wb") as w:
                    shutil.copyfileobj(r, w)
    elif dir.name == "LocalizeData":
        for ldir in dir.glob('*'):
                for idir in ldir.glob('*'):
                    os.makedirs(f'{orig}\\{dir.name}\\{ldir.name}\\{idir.name}', exist_ok=True)
                    for file in idir.glob('*'):
                        dest = f'{orig}\\{dir.name}\\{ldir.name}\\{idir.name}\\{file.name}'
                        with open(file, "rb") as r, open(dest, "wb") as w:
                            shutil.copyfileobj(r, w)
    elif dir.is_dir():
        os.makedirs(f'{orig}\\{dir.name}', exist_ok=True)
        for file in dir.glob('*'):
            dest = f'{orig}\\{dir.name}\\{file.name}'
            with open(file, "rb") as r, open(dest, "wb") as w:
                shutil.copyfileobj(r, w)
    elif dir.is_file():
        with open(file, "rb") as r, open(f'{orig}\\{dir.name}', "wb") as w:
            shutil.copyfileobj(r, w)