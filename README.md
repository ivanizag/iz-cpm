# iz-cpm -- Portable CP/M 2.2 environment
Work in progress

CP/M Emulation. Emulation of the CP/M services on top of the host OS. Z80 CP/M binaries work directly with the host operating system file system.

Uses the [iz80](https://github.com/ivanizag/iz80) library for Z80 emulation.

## Features

- Execute 8080 and Z80 binaries on top of CP/M
- Uses directly the host computer filesystem
- Terminal emulation of ADM-3A as used in the KAYPRO computers
- Z80 emulation validated with ZEXALL
- Z80 execution tracing
- BDOS and BIOS tracing
- Portable anywhere Rust runs

## Commands

Run the command prompt CCP:
```
cargo run
```

Use the ASM80 assembler:
```
cd src/rom
cargo run ASM.COM FILE.ASM
```

Run Tiny Basic:
```
cargo run diskA/TINYBAS.COM
```

Run the ZEXALL test suite (very long, better using release mode):
```
cargo run --release diskA/zexall.com
```

## Additional software

Download any CP/M 2.2 for Z80 bin. You can use `software/download.sh` to get a bunch of stuff.

Run Zork 1, Zork 2 or Zork 3:
```
cd software/zork
cargo run ZORK1.COM
```

Run Microsoft Basic:
```
cd software
cargo run MBASIC.COM # v5.29 from 1985
cargo run OBASIC.COM # v4.51 from 1977
```

Run LISP/80 vesion 1.1:
```
cd software/lisp80
cargo run LISP_80.COM
```

Run MULISP-80 ([manual](http://www.retroarchive.org/docs/mulisp_mustar.pdf)):
```
cd software/lisp80
cargo run MULISP.COM
```

## TODO

- Complete BIOS
- Complete BDOS
- Manage disks as separate folders
- BIOS input from host console should not be echoed by the host
- Reset and boot after program execution
