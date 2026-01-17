<a href="https://kkratterf.github.io/fratmscript/">
  <img alt="JavaScript ma comme si deve" src="https://github.com/kkratterf/fratmscript/packages/playground/public/cover.jpeg">
</a>

# FratmScript 🤌🏻

**JavaScript ma comme si deve** (JavaScript, the way it should be)

A transpiler that converts Neapolitan code to JavaScript. Written in Rust.

```fratm
chist è nome = "Gennaro"

facc saluta(chi) {
    si (chi == nisciun) {
        piglie "E chi si tu?"
    }
    piglie "Uè " + chi + "!"
}

stamm a dì(saluta(nome))
```

---

## Monorepo Structure

```
fratmscript/
├── crates/
│   ├── fratm-core/       # Core compiler (lexer, parser, codegen)
│   ├── fratm-cli/        # CLI tool
│   └── fratm-wasm/       # WebAssembly bindings
├── packages/
│   ├── vscode-extension/ # Syntax highlighting
│   └── playground/       # Online editor
├── examples/             # Example programs
└── docs/                 # Documentation
```

---

## Quick Start

```bash
# Clone
git clone https://github.com/kkratterf/fratmscript
cd fratmscript

# Build
cargo build --release

# Run
./target/release/fratm run examples/01_salutatore.fratm

# REPL
./target/release/fratm repl
```

---

## CLI Commands

```bash
fratm run <file.fratm>              # Compile and run
fratm build <file.fratm>            # Compile to JavaScript
fratm build <file> --sourcemap      # With source map
fratm repl                          # Interactive REPL
fratm tokens <file>                 # Debug: show tokens
fratm ast <file>                    # Debug: show AST
```

---

## Complete Syntax

### Variables
```fratm
chist è costante = 42        // const
tien variabile = "ciao"      // let
```

### Functions
```fratm
facc somma(a, b) {
    piglie a + b
}

mo vir facc caricaDati() {   // async
    chist è dati = aspett fetch(url)
    piglie dati
}
```

### Control Flow
```fratm
si (cond) { } sinnò { }       // if/else
mentre che (cond) { }         // while
pe (init; cond; upd) { }      // for (ogni is optional)
rompe                         // break
salta                         // continue
```

### Values
```fratm
overo / sfòls                // true/false
nisciun                      // null
boh                          // undefined
stu cos                      // this
```

### Operators
```fratm
e / o / no                   // && / || / !
pure                         // && (alias for "e")
manco                        // ! (alias for "no")
!                            // ! (direct)
```

### Classes
```fratm
na famiglie Persona {
    facc costruttore(nome) {
        stu cos.nome = nome
    }
}
chist è p = nu bell Persona("Gennaro")
```

### Try/Catch
```fratm
pruvamm {
    // ...
} e si schiatta (err) {
    // ...
}
iett nu bell Error("message")
```

### Modules
```fratm
chiamm { x } da "module"     // import
mann for facc fn() { }       // export
mann for predefinit App      // export default
```

### Console Methods
```fratm
stamm a dì(msg)              // console.log
avvis a dì(msg)              // console.warn
scrive a dì(msg)             // console.error
```

### New Features
```fratm
leva oggetto.prop            // delete
fermete                      // debugger
```

---

## Keyword Reference

| FratmScript | JavaScript | Neapolitan |
|-------------|------------|------------|
| `chist è` | `const` | "this is" |
| `tien` | `let` | "hold" |
| `facc` | `function` | "I do" |
| `piglie` | `return` | "take" |
| `si` | `if` | "if" |
| `sinnò` | `else` | "otherwise" |
| `mentre che` | `while` | "while" |
| `pe` | `for` | "for" |
| `overo` | `true` | "true" |
| `sfòls` | `false` | "false" |
| `nisciun` | `null` | "nobody" |
| `boh` | `undefined` | "dunno" |
| `stamm a dì` | `console.log` | "we say" |
| `avvis a dì` | `console.warn` | "warn" |
| `scrive a dì` | `console.error` | "write" |
| `mo vir` | `async` | "now see" |
| `aspett` | `await` | "wait" |
| `pruvamm` | `try` | "let's try" |
| `e si schiatta` | `catch` | "if it breaks" |
| `iett` | `throw` | "throw" |
| `nu bell` | `new` | "a nice" |
| `na famiglie` | `class` | "a family" |
| `stu cos` | `this` | "this thing" |
| `chiamm` | `import` | "call" |
| `da` | `from` | "from" |
| `mann for` | `export` | "send out" |
| `e` / `pure` | `&&` | "and" / "also" |
| `o` | `\|\|` | "or" |
| `no` / `manco` / `!` | `!` | "no" / "not even" |
| `leva` | `delete` | "remove" |
| `fermete` | `debugger` | "stop" |

---

## Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build WASM
cd crates/fratm-wasm
wasm-pack build --target web

# Package VSCode extension
cd packages/vscode-extension
vsce package
```

---

## Playground

The playground is a web-based editor to try FratmScript in your browser.

```bash
# Build WASM module
cd crates/fratm-wasm
wasm-pack build --target web --out-dir ../../packages/playground/pkg

# Serve the playground
cd ../../packages/playground
python3 -m http.server 8080

# Open http://localhost:8080
```

The playground also works in demo mode without WASM using a regex-based transpiler.

---

## Examples

Check the `examples/` folder for complete programs:

1. `01_salutatore.fratm` - Hello World
2. `02_variabili_matematica.fratm` - Variables and operations
3. `03_condizionali.fratm` - If/else
4. `04_loop.fratm` - While and for
5. `05_array_oggetti.fratm` - Data structures
6. `06_funzioni.fratm` - Advanced functions
7. `07_async.fratm` - Async/await
8. `08_classi.fratm` - OOP
9. `09_moduli.fratm` - Import/export
10. `10_nuove_feature.fratm` - New features

---

## Contributing

1. Fork it
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

---

## License

MIT

---

<p align="center">
  <b>Made with love and coffee</b>
</p>
