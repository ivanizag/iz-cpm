# iz-cpm
Work in process

CP/M Emulation. Emulation of the CP/M services on top of the host OS. Z80 CP/M binaries work directly with the host operating system file system.

Uses the [iz80](https://github.com/ivanizag/iz80) library for Z80 emulation.

## Commands

Use the ASM80 assembler:
```
cd src/rom
cargo run ASM.COM FILE.ASM
```

Run Tiny Basic:
```
cargo run diskA/TINYBAS.COM
```

Run Zork 1, Zork 2 or Zork 3:
```
cd diskA
cargo run ZORK1.COM
```

Run LISP/80 vesion 1.1:
```
cd diskA/Lisp-80
cargo run LISP_80.COM
```

Run MULISP-80 ([manual](http://www.retroarchive.org/docs/mulisp_mustar.pdf)):
```
cd diskA/Lisp-80
cargo run MULISP.COM
```

Run the ZEXALL test suite (very long, better compiling in release mode):
```
cargo run --release diskA/zexall.com
```

