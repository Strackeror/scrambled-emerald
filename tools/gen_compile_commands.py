import json
import pathlib

home = pathlib.Path(".").resolve()
build = home / 'build'
src = home / 'src'

c_flags = [
    'arm-none-eabi-gcc',
    '-mthumb',
    '-O2',
    '-mabi=apcs-gnu',
    '-mtune=arm7tdmi',
    '-march=armv4t',
    '-Wno-pointer-to-int-cast',
    '-std=gnu17',
    '-Werror',
    '-Wall',
    '-Wno-strict-aliasing',
    '-Wno-attribute-alias',
    '-Woverride-init',
    '-include global.h',
    '-D__CLANGD__',
]
c_commands = [
    {
        'directory': build,
        'arguments': c_flags + [
            f'-I{home}/include',
            f'-I{home}/gflib',
            '-o',
            file.with_suffix('.o'),
            file.resolve()
        ],
        'file': file.resolve()
    } for file in src.rglob('*.c')
]

with open('compile_commands.json', 'w') as outfile:
    json.dump(c_commands, outfile, default=str, indent=4)
