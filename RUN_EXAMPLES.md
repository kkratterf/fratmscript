# Run Guide: Examples, Games, Scripts

This guide shows how to quickly run the FratmScript content in this repository.

## Install `just` (optional but recommended)

`just` is the task runner used in this project. Install it based on your OS:

### Linux

```bash
# Ubuntu/Debian (if available in your repo)
sudo apt install just

# Fedora
sudo dnf install just

# Arch
sudo pacman -S just
```

### macOS

```bash
brew install just
```

### Windows

```powershell
winget install Casey.Just
# or
scoop install just
# or
choco install just
```

### Universal alternative (if Rust is already installed)

```bash
cargo install just
```

Official `just` documentation:
- https://just.systems/man/en/
- https://github.com/casey/just#installation

## Prerequisites

```bash
# From the project root
cargo build --release -p fratm-cli
```

Commands below use `./target/release/fratm`.

## Run a single example

```bash
./target/release/fratm lancia examples/01_salutatore.fratm
./target/release/fratm lancia examples/11_oop_avanzato.fratm
```

## Run all examples

```bash
for f in examples/*.fratm; do
  ./target/release/fratm lancia "$f"
done
```

## Run games

```bash
./target/release/fratm lancia Games/Pomodor_e_Muzzarel/pomodor_e_muzzarel.fratm
```

## Run extra scripts (launcher menu)

```bash
./target/release/fratm lancia PersonalScripts/run_personal_scripts.fratm
```

In the menu:
- `1` opens PDF Manager (you can pass arguments, for example `help`).
- `2` starts the Pomodor e Muzzarel game.
- `q` exits.

## Alternative with `just`

This project’s `justfile` is OS-aware:
- it uses `bash` on Linux/macOS
- it uses `PowerShell` on Windows
- it automatically resolves the release binary (`fratm` vs `fratm.exe`)
- running `just` without arguments shows the full menu with recipe descriptions

```bash
just
just release
just examples-lancia
just personal-lancia
just lancia-release examples/11_oop_avanzato.fratm
```

## Quick troubleshooting

- If `fratm` does not start: rebuild with `cargo build --release -p fratm-cli`.
- If PDF Manager cannot find dependencies: check `PersonalScripts/pdf_manager/node_modules`.
