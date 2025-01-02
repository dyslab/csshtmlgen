# csshtmlgen: Html Boilerplate Generator With CSS Framework

[![Rust-1.83.0](./assets/rust-1.83.0-darkgrey-for-the-badge.svg)](https://www.rust-lang.org/)

## Development

Dev. OS: Windows 10 / 11

Dev. Tools: [VS Code](https://code.visualstudio.com/)

Programming language & Build system: [Rust](https://www.rust-lang.org/) & [Cargo](https://doc.rust-lang.org/cargo/index.html)

Supported CSS Frameworks: [Bulma](https://bulma.io/) / [Simple.css](https://simplecss.org/)

## Build

```powershell
cargo build --release
# Expected: target/release/csshtmlgen.exe
# The artifact bases on the local host CPU architecture and operating system

# Below artifacts base on the specified CPU architecture and operating system

cargo build --release --target "x86_64-pc-windows-msvc"
# Expected: target/x86_64-pc-windows-msvc/release/csshtmlgen.exe

cargo build --release --target "x86_64-unknown-linux-musl"
# Expected: target/x86_64-unknown-linux-musl/release/csshtmlgen

cargo build --release --target "aarch64-apple-darwin"
# Expected: target/aarch64-apple-darwin/release/csshtmlgen

# Or, use the following command to build all artifacts at once in case you have 
# multi-architecture libraries installed successfully by Rustup
# cargo build --release --target "x86_64-pc-windows-msvc" --target "x86_64-unknown-linux-musl" --target "aarch64-apple-darwin"
```

## Usage

- On Windows (based on CPU architecture: x86_64-pc-windows-msvc):

```powershell
# Execute in the PowerShell / Cmd terminal, or double-click it in file explorer directly
.\target\x86_64-pc-windows-msvc\release\csshtmlgen.exe
```

- On Linux (based on CPU architecture: x86_64-unknown-linux-musl):  _**Yet to do!**_

```bash
# Execute in the Bash terminal, `chmod 755 ./target/x86_64-unknown-linux-musl/release/csshtmlgen` first if you have not done so
./target/x86_64-unknown-linux-musl/release/csshtmlgen
```

- On MacOS (based on CPU architecture: aarch64-apple-darwin): _**Yet to do!**_

```zsh
# Execute in the Zsh terminal, `chmod 755 ./target/aarch64-apple-darwin/release/csshtmlgen` first if you have not done so
./target/aarch64-apple-darwin/release/csshtmlgen
```
