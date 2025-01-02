# csshtmlgen: Html Boilerplate Generator With CSS Framework

[![Rust-1.83.0](./assets/rust-1.83.0-darkgrey-for-the-badge.svg)](https://www.rust-lang.org/)

## Development

Dev. OS: Windows 10 / 11

Dev. Tools: [VS Code](https://code.visualstudio.com/)

Programming language & Build system: [Rust](https://www.rust-lang.org/) & [Cargo](https://doc.rust-lang.org/cargo/index.html)

Supported CSS Frameworks: [Bulma](https://bulma.io/) / [Tailwind](https://tailwindcss.com/) / [Bootstrap](https://getbootstrap.com/) / [Simple.css](https://simplecss.org/)

## Build from the source

Fork the repository or download the zip archive of source codes, then build locally

```powershell
cargo build --release
# Expected: target/release/csshtmlgen.exe
# The artifact bases on the local host CPU architecture and operating system

# Below artifacts base on the specified CPU architecture and operating system

cargo build --release --target "x86_64-pc-windows-msvc"
# Expected: target/x86_64-pc-windows-msvc/release/csshtmlgen.exe

cargo build --release --target "x86_64-unknown-linux-gnu"
# Expected: target/x86_64-unknown-linux-gnu/release/csshtmlgen

cargo build --release --target "aarch64-apple-darwin"
# Expected: target/aarch64-apple-darwin/release/csshtmlgen

# Or, use the following command to build all artifacts at once in case you have 
# multi-architecture libraries installed successfully by Rustup
# cargo build --release --target "x86_64-pc-windows-msvc" --target "x86_64-unknown-linux-gnu" --target "aarch64-apple-darwin"
```

> NOTE: the project was manually built and released by the Github actions for demonstration purpose.

## Usage

- On Windows (based on CPU architecture: x86_64-pc-windows-msvc):

```powershell
# Execute in the PowerShell / Cmd terminal, or double-click it in file explorer directly
.\target\x86_64-pc-windows-msvc\release\csshtmlgen.exe
```

- On Linux (based on CPU architecture: x86_64-unknown-linux-gnu):

```bash
# Execute in the Bash terminal, `chmod 755 ./target/x86_64-unknown-linux-gnu/release/csshtmlgen` first if you have not done so
./target/x86_64-unknown-linux-gnu/release/csshtmlgen
```

- On MacOS (based on CPU architecture: aarch64-apple-darwin):

```zsh
# Execute in the Zsh terminal, `chmod 755 ./target/aarch64-apple-darwin/release/csshtmlgen` first if you have not done so
./target/aarch64-apple-darwin/release/csshtmlgen
```
