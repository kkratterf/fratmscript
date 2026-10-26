<a href="https://kkratterf.github.io/fratmscript/">
  <img alt="JavaScript ma comme si deve" src="https://github.com/kkratterf/fratmscript/blob/main/packages/playground/public/cover.jpeg">
</a>

# FratmScript 🤌🏻

**JavaScript ma comme si deve** (JavaScript, the way it should be)

[![CI](https://github.com/kkratterf/fratmscript/actions/workflows/ci.yml/badge.svg)](https://github.com/kkratterf/fratmscript/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/kkratterf/fratmscript)](https://github.com/kkratterf/fratmscript/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](#license)

A transpiler that converts Neapolitan code to JavaScript. Written in Rust.

### 👉 [Prova 'o playground](https://kkratterf.github.io/fratmscript/) (try it in the browser, with a 12-lesson tour)

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

▶️ [Prova stu codice int'o playground](https://kkratterf.github.io/fratmscript/#code=MYCwlgzgLgBAFzAdgewLYFMYF4YCIDi6iiAhgE7K4BQVAZicMDBCQDYCuUJAFKGAJQwA3lRhjmYGL3DYciSMDDtEgkePUwADmADmrMJlwBRGHwkwo7APzV1AX1Hjteg3gCqCXDADUpmb9wAQmoHKmgSVFQYEhgAEwAb7hYOLm4UDH5+KiA)

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
│   ├── agent-plugin/     # Claude Code plugin: skill + MCP server
│   └── playground/       # Online editor
└── examples/             # Example programs
```

---

## Install

No install needed to try it: open the [playground](https://kkratterf.github.io/fratmscript/).

To get the `fratm` CLI (it runs programs with Node.js):

```bash
# macOS and Linux (x64)
curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.ps1 | iex
```

The script downloads the binary for your system from the [latest release](https://github.com/kkratterf/fratmscript/releases/latest), checks its SHA-256 and puts it in `~/.local/bin` (`~\.fratm\bin` on Windows; `FRATM_INSTALL_DIR` changes it, `FRATM_VERSION=v0.5.0` picks a version). You can also download the archive from the release page yourself.

Then:

```bash
fratm lancia ciao.fratm   # compile and run
fratm repl                # interactive REPL
```

### Build from source

```bash
cargo install --git https://github.com/kkratterf/fratmscript fratm-cli

# or, from a clone
git clone https://github.com/kkratterf/fratmscript
cd fratmscript
cargo build --release
./target/release/fratm lancia examples/01_salutatore.fratm
```

---

## npm and Vite

The `fratmscript` npm package ships the compiler as WebAssembly with a small JS API and a Vite plugin that lets you `import` `.fratm` files. See [`packages/fratmscript`](packages/fratmscript/README.md).

```js
// vite.config.js
import fratmscript from 'fratmscript/vite';
export default { plugins: [fratmscript()] };
```

Without the CLI, `npm create fratm@latest pizzeria` writes the same project (the `create-fratm` package, same templates and options).

`fratm nuovo pizzeria` creates a ready-made React app written in FratmScript: Vite with this config, `src/App.fratm` with JSX and `useState`, a component with a CSS module and a stylesheet. Then `cd pizzeria && npm install && npm run dev`. `fratm nuovo --completa` adds the back end: a [Hono](https://hono.dev) API in `server/api.fratm` that keeps its data in SQLite with `apri_db` and `gomorra` queries, served by the same `npm run dev` and by `npm start` in production (Node 22.13+). `--tailwind` (also with `--completa`) adds [Tailwind CSS](https://tailwindcss.com) v4 with its Vite plugin: utility classes in `className="…"` work in every `.fratm` file. `fratm nuovo --senza-react` makes the Vite project without React (plain DOM code), and `--semplice` just a `main.fratm`. The generated `package.json` takes `fratmscript` from npm, at the version of your `fratm` (`^X.Y.Z`).

Server code runs in Node without the `fratm` binary: `node --import fratmscript/node server.fratm` compiles each `.fratm` module as it is loaded.

Or, with no build step at all, in any HTML page:

```html
<script type="module" src="https://cdn.jsdelivr.net/npm/fratmscript/browser.js"></script>
<script type="text/fratmscript">
  stamm a dì("Uè!")
</script>
```

---

## For AI agents

FratmScript is ready for coding agents (Claude Code, Cursor, Codex, Copilot...), both to discover it and to write it correctly:

- **Reference for LLMs**: [`llms.txt`](https://kkratterf.github.io/fratmscript/llms.txt) and the full [`llms-full.txt`](https://kkratterf.github.io/fratmscript/llms-full.txt) (rules that differ from JavaScript, every construct with an example, error glossary), plus the keyword map [`keywords.json`](https://kkratterf.github.io/fratmscript/keywords.json). `fratm guida` prints the same reference offline.
- **MCP server**: `fratm mcp` gives agents the tools `fratm_reference`, `fratm_check`, `fratm_compile`, `fratm_run`, `fratm_format` and `fratm_gomorra` (checks a GomorraSQL query), with errors as JSON (line, column, hint). In Claude Code: `claude mcp add fratm -- fratm mcp`.
- **Claude Code plugin** with a `fratmscript` skill and the MCP server:

  ```text
  /plugin marketplace add kkratterf/fratmscript
  /plugin install fratmscript@fratmscript
  ```

  The skill is a plain [Agent Skill](packages/agent-plugin/skills/fratmscript/SKILL.md), usable by other agents too.
- **JSON from the CLI**: `fratm check --json`, `fratm build --json`, `fratm lancia --json --timeout 10 -` (see below).

---

## CLI Commands

```bash
fratm nuovo <folder>                # New React + Vite app (--completa: + API and database, --tailwind: + Tailwind CSS, --senza-react: no React, --semplice: just main.fratm)
fratm lancia <file.fratm>           # Compile and launch
fratm build <file.fratm>            # Compile to JavaScript
fratm build <file> --sourcemap      # With source map
fratm check <files...>              # Report errors without writing output
fratm fmt <files...>                # Format files in place (comments kept)
fratm gomorra "<query>"             # Print the SQL of a GomorraSQL query (--db file.db: run it, or open a shell)
fratm fmt --check <files...>        # Fail if a file is not formatted
fratm repl                          # Interactive REPL
fratm guida                         # Language reference for AI agents
fratm mcp                           # MCP server for AI agents (stdio)
fratm tokens <file>                 # Debug: show tokens
fratm ast <file>                    # Debug: show AST
```

`check`, `build` and `lancia` take `--json` for one machine-readable JSON object (errors with line, column and hint; `build` adds the JavaScript, `lancia` the captured output and runtime error). `-` as the file reads the program from stdin, `build --stdout` prints the JavaScript and `lancia --timeout <seconds>` stops programs that never end:

```bash
echo 'stamm a dì(1 + 1)' | fratm lancia --json -
# {"errors":[],"exitCode":0,"ok":true,"runtimeError":null,"stderr":"","stdout":"2\n"}
```

A `.fratm` file can start with a shebang line and be run as a script:

```fratm
#!/usr/bin/env -S fratm lancia
stamm a dì("Uè!")
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

chist è triplo = facc (x) { piglie x * 3 }   // function expression (also named, mo vir, facc*)
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
[a, b] = [b, a]                // assignment to existing variables
({ x, y: punto.y } = origine)  // objects need ( ) at the start of a line, as in JS
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
    mo vir carica() { },         // async method
    damme prezzo() { piglie 8 }, // getter (miette for a setter)
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
fore: pe (…) { rompe fore }   // labelled break/continue
```

### Long expressions
```fratm
chist è totale = prezzo +            // a line ending with an operator goes on
    tasse
chist è nomi = persone
    .filter(p => p.età > 18)         // so does a line starting with .method()
    .map(p => p.nome)
chist è x = cond
    ? "sì"
    : "no"
```
Inside `( )` and `[ ]` line breaks never matter.

### Values
```fratm
`Uè ${nome}!`                // template literal (multi-line, nested ${ })
String.raw`a\nb`             // tagged template
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
"nome" dint'a pizza          // "nome" in pizza (also #privato dint'a x in classes)
a & b  a | b  a ^ b  ~a      // bitwise, also << >> >>> and &= |= ^= <<= >>= >>>=
+x  void 0                   // unary plus, void
10n  0xFFn                   // BigInt literals
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
    fisso { }                    // static initialization block
}

chist è Animale = na famiglie { }            // class expression (also named, with figlio 'e)
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
chiamm { x, y comme z } da "module"   // import { x, y as z }
chiamm React da "react"               // default import
chiamm React, { useState } da "react"
chiamm * comme fs da "node:fs"        // import * as fs
chiamm "./stile.css"                  // import for side effects only
chist è m = aspett chiamm("./m.js")   // dynamic import()

mann for facc fn() { }                // export
mann for predefinit App               // export default
mann for { a, b comme c }             // export { a, b as c }
mann for { a } da "./altro.js"        // re-export
mann for * da "./altro.js"            // export * (also * comme ns)
```

`comme` (or `as`) is a keyword only between a module name and its alias, and `predefinit` there means `default`: `chiamm { predefinit comme App } da "./app.js"`.

### JSX: React components in Neapolitan
```fratm
chiamm { useState } da "react"
chiamm "./stile.css"

mann for predefinit facc Contatore() {
    tien [pizze, setPizze] = useState(0)
    piglie (
        <core class="pizzeria">
            <titolo>Pizze sfurnate: {pizze}</titolo>
            <buttone onClick={() => setPizze(pizze + 1)}>Sforna</buttone>
        </core>
    )
}
```

JSX compiles to the automatic runtime of React (`react/jsx-runtime`). Write `// @jsxImportSource preact` at the top of a file to use Preact instead. HTML tags have Neapolitan names, translated at compile time so the browser still gets standard HTML: `scatola` → `div`, `piezzo` → `span`, `scritta` → `p`, `titolo` → `h1`, `titolo2` → `h2`, `titolo3` → `h3`, `titolo4` → `h4`, `titolo5` → `h5`, `titolo6` → `h6`, `buttone` → `button`, `casella` → `input`, `quaderno` → `textarea`, `scegli` → `select`, `opzione` → `option`, `etichetta` → `label`, `modulo` → `form`, `lista` → `ul`, `elenco` → `ol`, `voce` → `li`, `collegamento` → `a`, `immagine` → `img`, `capa` → `header`, `pede` → `footer`, `core` → `main`, `sezione` → `section`, `articolo` → `article`, `navigazione` → `nav`, `acapo` → `br`, `linea` → `hr`, `forte` → `strong`, `corsivo` → `em`, `codice` → `code`, `tabella` → `table`, `riga` → `tr`, `cella` → `td`, `capocella` → `th`, `tela` → `canvas`. HTML names (`div`, `button`) work too, a capitalised tag is a component, `class` and `for` work on every HTML tag, and CSS is plain CSS (or CSS modules) imported through Vite. A `<` starts JSX only where a value can start and a tag name follows right away, so `a < b` is still a comparison.

### The `fratm` library

Small helpers with Neapolitan names. Import the ones you use: the compiler writes their definitions into the output (no package needed), and the rest cost nothing.

```fratm
chiamm { a_caso, mischia, quanti, aspetta } da "fratm"
```

| Helper | What it does |
|--------|--------------|
| `a_caso(min, max)` | random integer from `min` to `max`, both included |
| `uno_a_caso(lista)` | random item of a list |
| `mischia(lista)` | shuffled copy of a list |
| `quanti(cosa)` | how many items: list or string length, Set/Map size, object keys |
| `arrotonda(n, cifre = 0)` | rounds `n` to `cifre` decimal digits |
| `ripeti(volte, fa)` | calls `fa(0)`, `fa(1)`, … `volte` times |
| `aspetta(ms)` | a Promise that resolves after `ms` milliseconds: `aspett aspetta(1000)` |
| ``gomorra`…` `` | a GomorraSQL query, see below |
| `apri_db(file)` | opens a SQLite database (Node 22.13+), see below |
| ``stile`…` `` | CSS in Neapolitan, see [below](#css-in-neapolitan-with-stile) |

### CSS in Neapolitan with `stile`

`` stile`…` `` (imported from `"fratm"`) writes CSS in Neapolitan. The compiler translates it when it compiles the file, so the browser gets standard CSS, and a missing `}` or a declaration without `:` is a compile error with its line. In a browser the rules are added to the page once; the call also returns the CSS text (in Node it only returns it).

```fratm
chiamm { stile } da "fratm"

chist è accento = "#c8321e"

stile`
  core.pizzeria { sfunno-culore: janco; spazio: 16px; mostra: flessibile }
  buttone { culore: ${accento}; bordo-tunno: 6px; cursore: manella }
  buttone:passanno { culore: russo !importante }
  @quanno schermo e (larghezza-massima: 600px) {
    scatola { larghezza: 100% }
  }
`
```

becomes `main.pizzeria { background-color: white; padding: 16px; display: flex }`, `button:hover { color: red !important }`, `@media screen and (max-width: 600px) { div { width: 100% } }`, and so on. The Neapolitan tag names of JSX work in selectors. Words that are not in the tables (class names, ids, units, colours like `#fff`, CSS properties in English) pass through, so plain CSS works too, and `${…}` values are kept. Strings, comments and `url(…)` are never translated.

- **Properties:** `culore` color, `sfunno` background, `sfunno-culore` background-color, `sfunno-immagine` background-image, `sfunno-grannezza` background-size, `scrittura` font-family, `scrittura-grannezza` font-size, `scrittura-peso` font-weight, `scrittura-stile` font-style, `allinea-testo` text-align, `decorazione-testo` text-decoration, `trasforma-testo` text-transform, `altezza-riga` line-height, `spazio-lettere` letter-spacing, `larghezza` width, `altezza` height, `larghezza-minima` min-width, `larghezza-massima` max-width, `altezza-minima` min-height, `altezza-massima` max-height, `margine` margin, `margine-ncoppa` margin-top, `margine-sotto` margin-bottom, `margine-manca` margin-left, `margine-destra` margin-right, `spazio` padding, `spazio-ncoppa` padding-top, `spazio-sotto` padding-bottom, `spazio-manca` padding-left, `spazio-destra` padding-right, `bordo` border, `bordo-culore` border-color, `bordo-larghezza` border-width, `bordo-stile` border-style, `bordo-tunno` border-radius, `bordo-ncoppa` border-top, `bordo-sotto` border-bottom, `bordo-manca` border-left, `bordo-destra` border-right, `contorno` outline, `mostra` display, `posizione` position, `ncoppa` top, `sotto` bottom, `manca` left, `destra` right, `livello` z-index, `trabocca` overflow, `visibilità` visibility, `visibilita` visibility, `opacità` opacity, `opacita` opacity, `ombra` box-shadow, `ombra-testo` text-shadow, `cursore` cursor, `spazio-mmiez` gap, `flessibile` flex, `direzione` flex-direction, `a-capo` flex-wrap, `cresce` flex-grow, `allinea-cose` align-items, `allinea-me` align-self, `giustifica` justify-content, `colonne` grid-template-columns, `righe` grid-template-rows, `area-griglia` grid-area, `transizione` transition, `trasforma` transform, `animazione` animation, `contenuto` content, `scatola` box-sizing, `lista-stile` list-style, `filtro` filter, `puntatore` pointer-events, `selezione` user-select, `galleggia` float, `adatta-oggetto` object-fit, `proporzione` aspect-ratio.
- **Values:** `russo` red, `janco` white, `niro` black, `nero` black, `verde` green, `blu` blue, `azzurro` skyblue, `celeste` lightblue, `giallo` yellow, `arancione` orange, `rosa` pink, `viola` purple, `grigio` gray, `marrone` brown, `oro` gold, `argiento` silver, `trasparente` transparent, `culore-corrente` currentcolor, `nisciuno` none, `niente` none, `blocco` block, `in-linea` inline, `blocco-in-linea` inline-block, `flessibile` flex, `griglia` grid, `miezo` center, `mmiezo` center, `manca` left, `destra` right, `ncoppa` top, `sotto` bottom, `inizio` start, `fine` end, `spartuto` space-between, `attuorno` space-around, `stiracchiato` stretch, `riga` row, `colonna` column, `fisso` fixed, `fissa` fixed, `assoluto` absolute, `assoluta` absolute, `relativo` relative, `relativa` relative, `appiccecato` sticky, `appiccecata` sticky, `statico` static, `grassetto` bold, `corsivo` italic, `normale` normal, `sottolineato` underline, `sbarrato` line-through, `maiuscolo` uppercase, `minuscolo` lowercase, `nascosto` hidden, `visibile` visible, `scorre` scroll, `automatico` auto, `manella` pointer, `aspetta` wait, `ereditato` inherit, `iniziale` initial, `a-capo` wrap, `nun-va-a-capo` nowrap, `copre` cover, `contiene` contain, `ripete` repeat, `nun-ripete` no-repeat, `continuo` solid, `tratteggiato` dashed, `puntinato` dotted, `bordo-incluso` border-box, `facile` ease, `lineare` linear, `infinito` infinite, `alternato` alternate, and `!importante`.
- **Pseudo-classes and elements:** `passanno` hover, `premuto` active, `fuoco` focus, `fuoco-visibile` focus-visible, `disattivato` disabled, `spuntato` checked, `primmo-figlio` first-child, `urdemo-figlio` last-child, `vacante` empty, `radice` root, `primma` before, `doppo` after, `segnaposto` placeholder.
- **At-rules:** inside `@keyframes`, `da` and `a` are `from` and `to`; `@quanno` is `@media` (with `schermo` screen, `stampa` print, `tutto` all, `e` and).

### GomorraSQL: SQL in Neapolitan

`` gomorra`…` `` writes SQL in Neapolitan. The compiler translates each query when it compiles the file, so a typo in the query is a compile error with its line. Every `${…}` becomes a bound parameter and never ends up inside the SQL text, so there is no SQL injection. The result is a plain object that works with any driver: `sql` and `params` use `?` (SQLite, MySQL), `text` and `values` use `$1`, `$2`… (PostgreSQL `pg`).

```fratm
chiamm { gomorra } da "fratm"

chist è minimo = 6
chist è care = gomorra`
    ripigliammo nomme, prezzo mmiez 'a pizze
    arò prezzo >= ${minimo}
    mettimmo in fila pe prezzo a scennere
    nun cchiù 'e 3
`
// care.sql:    SELECT nomme, prezzo FROM pizze WHERE prezzo >= ? ORDER BY prezzo DESC LIMIT 3
// care.params: [6]
```

| GomorraSQL | SQL | GomorraSQL | SQL |
|------------|-----|------------|-----|
| `ripigliammo` | `SELECT` | `arò` | `WHERE` |
| `tutto chillo ch'era 'o nuostro` | `*` | `e`, `o`, `nun` | `AND`, `OR`, `NOT` |
| `mmiez 'a` | `FROM` | `è`, `nun è` | `IS`, `IS NOT` |
| `pesc e pesc` | `JOIN` | `nisciun` | `NULL` |
| `nzipp 'ngoppa … chist` | `INSERT INTO … VALUES` | `overo`, `sfòls` | `TRUE`, `FALSE` |
| `rifacimm … accunza … accussì` | `UPDATE … SET … =` | `assumiglia a` | `LIKE` |
| `facimm na' strage mmiez 'a` | `DELETE FROM` | `dint'a` | `IN` |
| `mettimmo in fila pe` | `ORDER BY` | `comme` | `AS` |
| `a salì`, `a scennere` | `ASC`, `DESC` | `facimm 'a tavula` | `CREATE TABLE IF NOT EXISTS` |
| `facimm 'e mucchie pe` | `GROUP BY` | `abbrucimm 'a tavula` | `DROP TABLE IF EXISTS` |
| `nun cchiù 'e` | `LIMIT` | `ua uagliò` | `BEGIN` |
| `zumpanno` | `OFFSET` | `iamme bello ia'`, `sfaccimm` | `COMMIT`, `ROLLBACK` |

Strings in a query go in `"…"`. Other words pass through as SQL: column names, `COUNT(*)`, `PRIMARY KEY`, `RETURNING`. A query may span lines and use parentheses, and `nzipp 'ngoppa pizze nomme, prezzo chist ${a}, ${b}` gets the parentheses SQL needs.

#### A database with `apri_db`

`apri_db("pizzeria.db")` opens a SQLite file with Node's built-in `node:sqlite` (Node 22.13 or newer, no npm package); `apri_db()` keeps the data in memory. It takes `gomorra` queries or plain SQL with `?` and its values.

```fratm
chiamm { apri_db, gomorra } da "fratm"

chist è db = apri_db("pizzeria.db")
db.esegui(gomorra`facimm 'a tavula pizze (id INTEGER PRIMARY KEY, nomme TEXT, prezzo REAL)`)
db.esegui(gomorra`nzipp 'ngoppa pizze nomme, prezzo chist ${"Margherita"}, ${5}`)
chist è care = aspett db.ripiglia(gomorra`ripigliammo nomme mmiez 'a pizze arò prezzo > ${4}`)
```

| Method | Returns |
|--------|---------|
| `db.ripiglia(q)` | every row, as a list of objects |
| `db.ripiglia_uno(q)` | the first row, or `nisciun` |
| `db.esegui(q)` | `{ cambiate, ultimo_id }`: rows changed, id of the last inserted row |
| `db.chiude()` | closes the database |

The methods are synchronous; writing `aspett` in front of them is fine and keeps the code working if a future driver is asynchronous. On older Node and in a plain browser page, `apri_db` throws an error that says so; the [playground](https://kkratterf.github.io/fratmscript/) runs it on [sql.js](https://sql.js.org) (SQLite in WebAssembly) with an in-memory database.

The keywords come from [GomorraSQL](https://github.com/aurasphere/gomorra-sql) by Donato Rimenti (MIT), a Java translator over JDBC; FratmScript ports its dialect to JavaScript and adds ordering, limits, grouping and tables.

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
| `comme` | `as` (imports/exports) | "like" |
| `e` / `pure` | `&&` | "and" / "also" |
| `o` | `\|\|` | "or" |
| `no` / `manco` / `!` | `!` | "no" / "not even" |
| `dint'a` | `in` (operator) | "inside" |
| `leva` | `delete` | "remove" |
| `fermete` | `debugger` | "stop" |
| `<scatola>…</scatola>`, `<Pizza />` | JSX (React/Preact); `scatola` = `div`, `scritta` = `p`, `buttone` = `button`, ... | markup in the code |
| ``gomorra`ripigliammo …` `` | SQL query with bound parameters ([GomorraSQL](#gomorrasql-sql-in-neapolitan)) | Gomorrah |
| ``stile`culore: russo` `` | CSS in Neapolitan ([`stile`](#css-in-neapolitan-with-stile)) | style |

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

The [playground](https://kkratterf.github.io/fratmscript/) is a web-based editor to try FratmScript in your browser. Press **Impara** (Learn, Scola) for *'A scola 'e FratmScript*, a tour of 12 short lessons with an exercise each, checked against the console output. Lessons live in `packages/playground/src/scola/` (starter, `.soluzione.fratm` and expected `.out`; CI runs every solution).

Press **Guida** (Guide) for the [language guide](https://kkratterf.github.io/fratmscript/#guida): every construct explained in Italian, English or Neapolitan, with examples you edit and run on the spot and their generated JavaScript. Link a section with `#guida=<id>`, e.g. [`#guida=cicli`](https://kkratterf.github.io/fratmscript/#guida=cicli). The guide lives in `packages/playground/src/guida/` (`guida.json` plus one `.fratm`/`.out` pair per example; CI runs every example).

Next to the generated JavaScript, the **Token** and **Tree** tabs show how the compiler reads your code; hover an item to highlight it in the editor.

**Share** copies a link with the code in the URL. The `</>` button next to it copies an `<iframe>` that embeds the playground (`?embed=1`: editor, output and Run only, nothing is saved in the visitor's browser), for docs and blog posts.

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
15. `15_libreria.fratm` - The `fratm` library: `a_caso`, `mischia`, `quanti`, `aspetta`…
16. `16_pizzeria_jsx.fratm` - A React component written with JSX
17. `17_gomorra.fratm` - GomorraSQL: SQL queries in Neapolitan with bound parameters
18. `18_magazzino.fratm` - A SQLite database with `apri_db` and `gomorra` (Node 22.13+, or the playground)
19. `19_pizzeria_cu_stile.fratm` - A React page all in Neapolitan: Neapolitan tags and CSS with `stile`

---

## Contributing

Contributions are welcome, from a new example program to a compiler fix.

- **Found a bug or have an idea?** [Open an issue](https://github.com/kkratterf/fratmscript/issues/new/choose): there are forms for bugs, features and new dialect words.
- **Want to start small?** Look for issues labelled [`good first issue`](https://github.com/kkratterf/fratmscript/labels/good%20first%20issue). A new program in `examples/` is always a good first PR (CI compiles every one of them).
- **Before a PR**, read [`AGENTS.md`](AGENTS.md): it lists the checks to run, the commit format and the rules for new language constructs. It is written for AI agents and humans alike.

---

## License

MIT

---

<p align="center">
  <b>Made with Claude and coffee</b>
</p>
