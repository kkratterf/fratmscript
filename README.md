<a href="https://kkratterf.github.io/fratmscript/">
  <img alt="JavaScript ma comme si deve" src="https://github.com/kkratterf/fratmscript/blob/main/packages/playground/public/cover.jpeg">
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
│   ├── fratmscript/      # npm package: WASM compiler + Vite plugin
│   ├── vscode-extension/ # Syntax highlighting
│   └── playground/       # Online editor
└── examples/             # Example programs
```

---

## Quick Start

```bash
# Clone
git clone https://github.com/kkratterf/fratmscript
cd fratmscript

# Build
cargo build --release

# Launch
./target/release/fratm lancia examples/01_salutatore.fratm

# REPL
./target/release/fratm repl
```

---

## npm and Vite

The `fratmscript` npm package ships the compiler as WebAssembly with a small JS API and a Vite plugin that lets you `import` `.fratm` files. See [`packages/fratmscript`](packages/fratmscript/README.md).

```js
// vite.config.js
import fratmscript from 'fratmscript/vite';
export default { plugins: [fratmscript()] };
```

---

## CLI Commands

```bash
fratm lancia <file.fratm>           # Compile and launch
fratm build <file.fratm>            # Compile to JavaScript
fratm build <file> --sourcemap      # With source map
fratm check <files...>              # Report errors without writing output
fratm fmt <files...>                # Format files in place (comments kept)
fratm fmt --check <files...>        # Fail if a file is not formatted
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
tien x = 1, y = 2            // several at once
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

chist è doppio = x => x * 2           // arrow functions
chist è somma = (a, b) => a + b
chist è carica = mo vir (url) => aspett fetch(url)

facc saluta(nome = "Gennà", ...altri) { }  // default and rest parameters
somma(...numeri)                          // spread in calls, [...a] and { ...o }

facc* conta(n) {                 // generator (function*)
    pe (tien i = 1; i <= n; i++) {
        caccia i                 // yield
    }
    caccia* altri()              // yield* (delegation)
}
mo vir facc* stream() { }        // async generator
// in classes and objects: facc* nomme() { } or *nomme() { }
```

### Destructuring
```fratm
chist è { nome, età: anni = 0, ...altro } = persona
tien [primo, , terzo, ...ato] = lista
facc saluta({ nome }) { }      // also in arrow and catch parameters
```

### Objects
```fratm
chist è pizza = {
    nome,                        // shorthand for nome: nome
    [chiave]: 1,                 // computed key
    "con spazi": 2,
    descrivi() {                 // method ("facc" optional)
        piglie "Pizza " + stu cos.nome
    },
    mo vir carica() { }          // async method
}
```

### Control Flow
```fratm
si (cond) { } sinnò { }       // if/else
vedimm (x) {                  // switch
    caso 1:                   // case
        rompe
    predefinit:               // default
}
mentre che (cond) { }         // while
primm fa { } mentre che (cond)   // do...while
pe (init; cond; upd) { }      // for (ogni is optional)
pe ogni (chist è x dint xs) { }    // for...of
pe ogni (tien k 'ncopp obj) { }    // for...in
pe aspett (chist è r dint stream) { }  // for await...of
rompe                         // break
salta                         // continue
```

### Values
```fratm
`Uè ${nome}!`                // template literal (multi-line, nested ${ })
/ab+/gi                      // regex literal
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
&& / ||                      // also accepted
x += 1  x -= 1  x *= 2       // also /=, %=, **=, ??=, ||=, &&=
i++  i--  ++i  --i           // increment / decrement
a ?? b                       // nullish coalescing
a?.b  a?.[k]  f?.()          // optional chaining
```

### Classes
```fratm
na famiglie Persona {
    costruttore(nome) {
        stu cos.nome = nome
    }
}

na famiglie Pizzaiolo figlio 'e Persona {
    costruttore(nome) {
        'o pate(nome)
    }

    fisso specie() {
        piglie "Essere umano"
    }
}

na famiglie Conto {
    #saldo = 0                   // private field
    fisso aperti = 0             // static field
    titolare = "nisciuno"        // field

    damme saldo() {              // get saldo()
        piglie stu cos.#saldo
    }
    miette saldo(valore) {       // set saldo(valore)
        stu cos.#saldo = valore
    }
    #controlla() { }             // private method
}
```

### Try/Catch
```fratm
pruvamm {
    // ...
} e si schiatta (err) {
    // ...
} aropp {
    // always runs (finally)
}
pruvamm { /* ... */ } aropp { /* ... */ }   // try/finally
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

| FratmScript | JavaScript | English |
|-------------|------------|------------|
| `chist è` | `const` | "this is" |
| `tien` | `let` | "hold" |
| `facc` | `function` | "I do" |
| `piglie` | `return` | "take" |
| `si` | `if` | "if" |
| `sinnò` | `else` | "otherwise" |
| `vedimm` / `caso` / `predefinit` | `switch` / `case` / `default` | "let's see" / "case" / "default" |
| `mentre che` | `while` | "while" |
| `primm fa … mentre che` | `do … while` | "first do … while" |
| `pe` | `for` | "for" |
| `dint` | `of` (for...of) | "inside" |
| `'ncopp` | `in` (for...in) | "on top of" |
| `overo` | `true` | "true" |
| `sfòls` | `false` | "false" |
| `nisciun` | `null` | "nobody" |
| `boh` | `undefined` | "dunno" |
| `stamm a dì` | `console.log` | "we say" |
| `avvis a dì` | `console.warn` | "warn" |
| `scrive a dì` | `console.error` | "write" |
| `mo vir` | `async` | "now see" |
| `aspett` | `await` | "wait" |
| `facc*` / `caccia` | `function*` / `yield` | "throw out" |
| `pruvamm` | `try` | "let's try" |
| `e si schiatta` | `catch` | "if it breaks" |
| `aropp` | `finally` | "afterwards" |
| `iett` | `throw` | "throw" |
| `nu bell` | `new` | "a nice" |
| `na famiglie` | `class` | "a family" |
| `figlio 'e` | `extends` | "child of" |
| `'o pate` | `super` | "the father" |
| `fisso` | `static` | "fixed/static" |
| `damme` | `get` (in classes) | "give me" |
| `miette` | `set` (in classes) | "put" |
| `stu cos` | `this` | "this thing" |
| `chiamm` | `import` | "call" |
| `da` | `from` | "from" |
| `mann for` | `export` | "send out" |
| `e` / `pure` | `&&` | "and" / "also" |
| `o` | `\|\|` | "or" |
| `no` / `manco` / `!` | `!` | "no" / "not even" |
| `leva` | `delete` | "remove" |
| `fermete` | `debugger` | "stop" |

No accents on your keyboard? `sinno`, `sfols`, `di` (in `stamm a di`), `chist e` and `chè e` work like `sinnò`, `sfòls`, `dì`, `chist è` and `chè è`; `typeof` and `instanceof` are accepted for `chè è` and `è uno`.

Short keywords (`e`, `o`, `no`, `da`, `de`, `na`, `nu`, `mo`, `che`, `cos`, `caso`, `pure`, `for`, `leva`, `scrive`) are keywords only where their construct can start, so they also work as names: `e si schiatta (e)`, `tien o = 1`, `facc f(da, a)`. `aropp`, `dint`, `vedimm`, `primm`, `fa`, `damme` and `miette` are not reserved at all (`damme`/`miette` mean get/set only before a class member name): they mean `finally` only right after a `pruvamm` block, `of` only in a `pe` header, `switch` only in `vedimm (…) {` and `do` only in `primm fa {`. Names that are reserved in JavaScript are renamed in the output (`tien for = 1` → `let $for = 1`).

---

## Development

Install `just`: https://just.systems/man/en/

```bash
# Recommended task runner
just --list

# Build and test
just build
just test

# Build/launch all examples
just examples-build
just examples-lancia

# Build release CLI
just release

# Build WASM for playground
just wasm
```

---

## Playground

The playground is a web-based editor to try FratmScript in your browser.

```bash
# Build WASM module
cd crates/fratm-wasm
wasm-pack build --target web --out-dir ../../packages/playground/public/pkg

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
9. `09_moduli.fratm` - Import/export without external deps
10. `10_nuove_feature.fratm` - New features
11. `11_oop_avanzato.fratm` - Inheritance, super and static methods
12. `12_trycatch_validazione.fratm` - Try/catch and validation
13. `13_pomodor_e_muzzarel.fratm` - Interactive tic-tac-toe in the terminal
14. `14_sintassi_moderna.fratm` - Arrows, templates, destructuring, spread, `?.`/`??`, regex

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
  <b>Made with Claude and coffee</b>
</p>
