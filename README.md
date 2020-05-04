# iz-cpm -- CP/M 2.2 environment

## What is this?

This is a CP/M 2.2 execution environment. It provides everything needed to run a standard CP/M for Z80 binary.

Uses my [iz80](https://github.com/ivanizag/iz80) library for Z80 emulation.

Made with Rust

## Installation
Extract the zip. Optionally run download.sh to download the CP/M system disk, Microsoft Basic, Turbo Pascal, Lisp and some games.

## Usage examples

Execute `iz-cpm` to open the CP/M command prompt (the CCP) on the current directory:
```console
casa@servidor:~/software/cpm22$ ls
ASM.COM      CCSINIT.COM   DISKDEF.LIB  iz-cpm        STAT.COM
CBIOS.ASM    CCSYSGEN.COM  DUMP.ASM     LOAD.COM      STDBIOS.ASM
CCBIOS.ASM   CPM24CCS.COM  DUMP.COM     MOVCPM.COM    SUBMIT.COM
CCBOOT.ASM   DDT.COM       ED.COM       PIP.COM       SYSGEN.COM
-CCSCPM.251  DEBLOCK.ASM   GENMOD.COM   RLOCBIOS.COM
casa@servidor:~/software/cpm22$ ../../iz-cpm 
iz-cpm https://github.com/ivanizag/iz-cpm
CP/M 2.2 Copyright (c) 1979 by Digital Research
Press ctrl-c ctrl-c to return to host

A>dir
A: CCSINIT  COM : MOVCPM   COM : CPM24CCS COM : STAT     COM
A: SYSGEN   COM : STDBIOS  ASM : IZ-CPM       : LOAD     COM
A: DISKDEF  LIB : ASM      COM : RLOCBIOS COM : DUMP     COM
A: -CCSCPM  251 : CCBIOS   ASM : DUMP     ASM : CCSYSGEN COM
A: DEBLOCK  ASM : ED       COM : SUBMIT   COM : DDT      COM
A: PIP      COM : CBIOS    ASM : CCBOOT   ASM : GENMOD   COM
A>
```

Execute `iz-cpm` with a file to execute a CP/M binary directly, bypassing the CPP:
```console
casa@servidor:~/code/z80/iz-cpm$ ./iz-cpm software/OBASIC.COM 
44531 Bytes free
BASIC Rev. 4.51
[CP/M Version]
Copyright 1977 (C) by Microsoft
Ok
print "hello"
hello
Ok

```

Map up to 16 directories as CP/M drives:
```console
casa@servidor:~$ ./iz-cpm --disk-a software/cpm22 --disk-b software/zork --disk-e .
iz-cpm https://github.com/ivanizag/iz-cpm
CP/M 2.2 Copyright (c) 1979 by Digital Research
Press ctrl-c ctrl-c to return to host

A>b:
B>dir
B: ZORK2    DAT : ZORK1    COM : ZORK3    DAT : ZORK3    COM
B: FILE_ID  DIZ : ZORK1    DAT : ZORK1    SAV : ZORK2    COM
B>zork1
ZORK I: The Great Underground Empire
Copyright (c) 1981, 1982, 1983 Infocom, Inc. All rights
reserved.
ZORK is a registered trademark of Infocom, Inc.
Revision 88 / Serial number 840726

West of House
You are standing in an open field west of a white house, with
a boarded front door.
There is a small mailbox here.

>
```

## Usage
```
iz-cpm https://github.com/ivanizag/iz-cpm
CP/M 2.2 Copyright (c) 1979 by Digital Research
Press ctrl-c ctrl-c to return to host 

USAGE:
    iz-cpm [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -t, --call-trace        Trace BDOS and BIOS calls
    -T, --call-trace-all    Trace BDOS and BIOS calls excluding screen I/O
    -z, --cpu-trace         Trace Z80 instructions execution
    -h, --help              Prints help information
    -s, --slow              Run slower
    -V, --version           Prints version information

OPTIONS:
    -a, --disk-a <path>    directory to map disk A: [default: .]
    -b, --disk-b <path>    directory to map disk B:
    -c, --disk-c <path>    directory to map disk C:
    -d, --disk-d <path>    directory to map disk D:
        --disk-e <path>    directory to map disk E:
        --disk-f <path>    directory to map disk F:
        --disk-g <path>    directory to map disk G:
        --disk-h <path>    directory to map disk H:
        --disk-i <path>    directory to map disk I:
        --disk-j <path>    directory to map disk J:
        --disk-k <path>    directory to map disk K:
        --disk-l <path>    directory to map disk L:
        --disk-m <path>    directory to map disk M:
        --disk-n <path>    directory to map disk N:
        --disk-o <path>    directory to map disk O:
        --disk-p <path>    directory to map disk P:

ARGS:
    <CMD>     The z80 image to run
    <ARGS>    Parameters for the given command
```

## Features

- Execution of 8080 and Z80 binaries on top of CP/M
- Direct usage of the host computer filesystem
- Terminal emulation of ADM-3A as used in the KAYPRO computers
- Z80 emulation validated with ZEXALL
- Z80 execution tracing
- BDOS and BIOS tracing
- Portable, runs in Linux, MacOS and Windows. Mmm, not in CP/M.

## How does it work

[CP/M](https://en.wikipedia.org/wiki/CP/M) was designed to be portable to a variety of devices using Intel 8080, Intel 8085 or Zilog Z80 processors thanks to a tiered architecture. The main parts of CP/M are:

- The BIOS: Basic Input/Output system. Interface with the hardware. CP/M defines a very small interface: 16 entrypoints to manage I/O and access to disk sectors. The hardware vendors provided the BIOS for their device based on source code available on the CP/M distribution.
- The BDOS: Basic Disk Operating System. Main component of CP/M. Provided by Digital Research and common to all systems. Provides the high level entrypoints to be used by application developpers.
- The CCP: Console Command Processor. The CP/M Command prompt.

To emulate this environment using the host filesystem, we have to provide a replacement BDOS translating as we don't want to relay on the physical disk sectors abstraction of the BIOS. The main components are:

- Z80 emulator. It uses [iz80](https://github.com/ivanizag/iz80)
- [BIOS](src/bios.rs) emulator. Only the I/O entrypoints. In theory, it shouldn't be necessary, but some programs use it directly, bypassing BDOS.
- [BDOS](src/bdos.rs) emulator. Traps the calls and executes code on the  host.
- CPP. No emulation needed. The CPP binary from CP/M 2.2 is used.
- [Terminal](src/terminal.rs) emulator. CP/M does not define how the terminal should work. Applications needed to be aware and usually could be configured for several leading options, like ADM-3a, VT-52, Hazeltine 1500 and Osborne. This emulator supports ADM-3a used also on the very popular Kaypro computers.

## Useful links:

- [The Unoffcial CP/M Web site](http://www.cpm.z80.de/)
- [CP/M Operating System Manual](http://www.gaby.de/cpm/manuals/archive/cpm22htm/)
- [CP/M Software in retroarchive.org](http://www.retroarchive.org/cpm/)

## TODO
- Proper documentation
- File level read-only option
- Verify in Mac
