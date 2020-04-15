# rust-z80
Work in progress

Runs Tiny Basic (version from [cpuville.com](http://cpuville.com/Code/Tiny-BASIC.html):
```
cargo run --bin cpuville
```

Passes the ZEXALL tests:
```
cargo test --release -- --nocapture --ignored
```

Initial support of CP/M binaries:
```
cargo run --bin cpm
```

