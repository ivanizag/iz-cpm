# iz-cpm
Work in process

CP/M Emulation. Emulation of the CP/M services on top of the host OS. Z80 CP/M binaries work directly with the host os FS.

## Commands


Run Tiny Basic :
```
cargo run src/rom/TINYBAS.COM
```

Run Zork 1, Zork 2 or Zork 3:
```
cd src/rom
cargo run ZORK1.COM
```

Run the ZEXALL test suite (very long, better compiling in release mode):
```
carg run --release src/rom/zexall.com
```

