set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

fratm_release_bin := if os_family() == "windows" { ".\\target\\release\\fratm.exe" } else { "./target/release/fratm" }

# Show the command menu (default behavior when running `just`).
[default]
[private]
menu:
  @just --justfile {{justfile()}} --list --unsorted

# Build Rust workspace (debug profile).
build:
  cargo build

# Build release CLI binary.
release:
  cargo build --release -p fratm-cli

# Run Rust tests.
test:
  cargo test

# Launch a FratmScript file with debug CLI (`cargo run`).
lancia file:
  cargo run -p fratm-cli -- lancia {{file}}

# Build a FratmScript file into JavaScript with debug CLI.
build-file file:
  cargo run -p fratm-cli -- build {{file}}

# Build all examples (`examples/*.fratm`).
examples-build:
  cargo run -p fratm-cli -- build examples/01_salutatore.fratm
  cargo run -p fratm-cli -- build examples/02_variabili_matematica.fratm
  cargo run -p fratm-cli -- build examples/03_condizionali.fratm
  cargo run -p fratm-cli -- build examples/04_loop.fratm
  cargo run -p fratm-cli -- build examples/05_array_oggetti.fratm
  cargo run -p fratm-cli -- build examples/06_funzioni.fratm
  cargo run -p fratm-cli -- build examples/07_async.fratm
  cargo run -p fratm-cli -- build examples/08_classi.fratm
  cargo run -p fratm-cli -- build examples/09_moduli.fratm
  cargo run -p fratm-cli -- build examples/10_nuove_feature.fratm
  cargo run -p fratm-cli -- build examples/11_oop_avanzato.fratm
  cargo run -p fratm-cli -- build examples/12_trycatch_validazione.fratm

# Launch all examples (`examples/*.fratm`).
examples-lancia:
  cargo run -p fratm-cli -- lancia examples/01_salutatore.fratm
  cargo run -p fratm-cli -- lancia examples/02_variabili_matematica.fratm
  cargo run -p fratm-cli -- lancia examples/03_condizionali.fratm
  cargo run -p fratm-cli -- lancia examples/04_loop.fratm
  cargo run -p fratm-cli -- lancia examples/05_array_oggetti.fratm
  cargo run -p fratm-cli -- lancia examples/06_funzioni.fratm
  cargo run -p fratm-cli -- lancia examples/07_async.fratm
  cargo run -p fratm-cli -- lancia examples/08_classi.fratm
  cargo run -p fratm-cli -- lancia examples/09_moduli.fratm
  cargo run -p fratm-cli -- lancia examples/10_nuove_feature.fratm
  cargo run -p fratm-cli -- lancia examples/11_oop_avanzato.fratm
  cargo run -p fratm-cli -- lancia examples/12_trycatch_validazione.fratm

# Launch a FratmScript file with the compiled release binary.
lancia-release file:
  {{fratm_release_bin}} lancia {{file}}

# Build WASM output for the playground.
wasm:
  cd crates/fratm-wasm; wasm-pack build --target web --out-dir ../../packages/playground/public/pkg

# Start playground dev server.
playground-dev:
  cd packages/playground; pnpm dev

# Build VSCode extension sources.
vscode-build:
  cd packages/vscode-extension; pnpm compile

# Open personal scripts launcher menu.
personal-lancia:
  cargo run -p fratm-cli -- lancia PersonalScripts/run_personal_scripts.fratm
