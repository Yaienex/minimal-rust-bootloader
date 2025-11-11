I was a mess for me to understand and implement the crate bootloader (especially the migration from v0.9 to v0.11). So I'm providing a minimal Rust project to have
a running "kernel". Be sure to have qemu install since that's what I use for my test !

## Troubleshooting
(Tell me if I miss something)
- Have the nightly version install and ready to go
I's already setup in the `rust-toolchain.toml` but you definetly need to have it
```bash
rustup install nightly
```
- You'll need the `x86_64-unknown-none` target
```bash
cargo add target x86-_64_unknown-none
```
That's it :D
