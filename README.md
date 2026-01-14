# 🏆 Codeforces Solutions and Template

[Codeforces](https://codeforces.com/) solutions, including high-performance Rust 1.89.0 boilerplate for quick initialisation and compile.

## Compilation

### Linux

```bash
rustc --edition=2024 -O -C link-arg=-Wl,-z,stack-size=268435456 --cfg ONLINE_JUDGE solution.rs

```

### Windows

```bash
rustc --edition=2024 -O -C link-args=/STACK:268435456 --cfg ONLINE_JUDGE solution.rs

```

### macOS

```bash
rustc --edition=2024 -O -C link-arg=-Wl,-stack_size,0x10000000 --cfg ONLINE_JUDGE solution.rs

```
## File Creation

To create file `a.rs` in directory `4`, and immediately open with `nvim`:

```
chmod +x init.sh
./init.sh 4A
```

Compile and run with:

```
chmod +x comp.sh
./comp.sh 4A
```

## Command Breakdown

- `--edition=2024`: Enables the latest Rust features.
- `-O`: Enables high-level optimizations for execution speed.
- **Stack size**: Sets the stack to 256MB using platform-specific linker flags to prevent overflows in recursive algorithms like DFS.
- `--cfg ONLINE_JUDGE`: Activates judge-specific code paths and markers.
