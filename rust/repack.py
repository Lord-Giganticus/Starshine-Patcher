import subprocess
from pathlib import Path

class FileDependency:
    def __init__(self, program: Path):
        if program.exists() != True:
            raise FileNotFoundError(f"{program.name} not found.")
        self.program = program.absolute()

    def execute(self, args: str):
        subprocess.run(f'{self.program} {args}')

WIT = FileDependency(Path("wit\\wit.exe"))

WIT.execute('copy smg2.d "Super Mario Starshine.wbfs" -ovv --id=....01 --name="Super Mario Starshine')