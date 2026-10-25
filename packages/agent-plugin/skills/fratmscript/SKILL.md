---
name: fratmscript
description: Write, check, run and debug FratmScript, JavaScript with Neapolitan keywords (.fratm files, `chist è`, `facc`, `piglie`, `stamm a dì`). Use when the user mentions FratmScript or .fratm files, asks for code "in napoletano", or works in the kkratterf/fratmscript repository.
---

# Writing FratmScript

FratmScript is JavaScript with Neapolitan keywords. The `fratm` compiler turns `.fratm` files into
modern JavaScript (ES modules). Operators, literals, method calls and every JavaScript built-in
(`Math`, `JSON`, `Promise`, `Map`, `fetch`, `process`...) stay exactly as in JavaScript: only the
keywords change.

## Workflow

1. Make sure the CLI is there: `fratm --version`. If not, install it:
   `curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh`
   (macOS/Linux; it goes to `~/.local/bin`) or
   `cargo install --git https://github.com/kkratterf/fratmscript fratm-cli`.
2. Read the full reference once per task: `fratm guida`, the `fratm_reference` tool of the
   `fratm` MCP server, or https://kkratterf.github.io/fratmscript/llms-full.txt.
3. Write the code, then always verify it before answering:
   - `fratm check --json file.fratm` (or the `fratm_check` tool) for compile errors;
   - `fratm lancia --json --timeout 10 file.fratm` (or `fratm_run`) to run it with Node.js.
   Both accept `-` to read the program from stdin.
4. Fix errors from the JSON: `line`, `column`, `message` (in Neapolitan) and `suggestion`.
   A suggestion like `In FratmScript 'const' se scrive 'chist è'` names the word to use.

## Rules that differ from JavaScript

- Never write `const`, `let`, `var`, `function`, `return`, `if`, `else`, `for`, `while`, `new`,
  `class`, `this`, `true`, `false`, `null`, `try`, `catch`, `throw`, `async`, `await`,
  `import`, `export`: use the table below.
- One statement per line (or `;`). An expression goes on to the next line after a binary
  operator, `=`, `?`, `:` or `=>`, before a line that starts with `.method()`, `?`, `:` or
  `&&`/`||`, and anywhere inside `( )` and `[ ]`.
- Braces are required: `si (x) { ... }`.
- `chist è` cannot be reassigned, and a name cannot be declared twice in the same scope.
- JSX works in `.fratm` files for React or Preact: `piglie <button onClick={f}>Uè</button>`.
  Tags stay HTML, hooks keep their React names (`useState`), `class` works on HTML tags. The
  output imports `react/jsx-runtime`; `// @jsxImportSource preact` switches library.
- Every modern JavaScript construct is supported; `fratm guida` lists the exact syntax.

| FratmScript | JavaScript |
| --- | --- |
| `chist è x = 1` / `tien y = 2` | `const x = 1` / `let y = 2` |
| `facc f(a) { piglie a }` | `function f(a) { return a }` |
| `(a) => a * 2` | same as JavaScript |
| `si (c) { } sinnò si (d) { } sinnò { }` | `if / else if / else` |
| `pe (tien i = 0; i < n; i++) { }` | `for (let i = 0; ...)` |
| `pe ogni (chist è x dint xs) { }` | `for (const x of xs)` |
| `pe ogni (chist è k 'ncopp o) { }` | `for (const k in o)` |
| `mentre che (c) { }` / `primm fa { } mentre che (c)` | `while` / `do ... while` |
| `vedimm (x) {` ... `caso 1:` ... `predefinit:` ... `}` | `switch / case / default` |
| `rompe` / `salta` | `break` / `continue` |
| `overo` / `sfòls` / `nisciun` / `boh` | `true` / `false` / `null` / `undefined` |
| `stamm a dì(x)` / `avvis a dì(x)` / `scrive a dì(x)` | `console.log` / `.warn` / `.error` |
| `mo vir facc f() { aspett p }` | `async function f() { await p }` |
| `facc* g() { caccia 1 }` | `function* g() { yield 1 }` |
| `pruvamm { } e si schiatta (e) { } aropp { }` | `try / catch (e) / finally` |
| `iett nu bell Error("...")` | `throw new Error("...")` |
| `na famiglie B figlio 'e A { costruttore() { 'o pate() } }` | `class B extends A { constructor() { super() } }` |
| `stu cos` / `fisso` / `damme` / `miette` | `this` / `static` / `get` / `set` |
| `chiamm { a } da "m"` | `import { a } from "m"` |
| `mann for facc f() { }` / `mann for predefinit x` | `export function` / `export default` |
| `e` / `o` / `no` (also `&&`, `\|\|`, `!`) | `&&` / `\|\|` / `!` |
| `chè è x` / `x è uno T` | `typeof x` / `x instanceof T` |
| `leva o.k` / `fermete` | `delete o.k` / `debugger` |

`chiamm { a_caso, mischia, quanti, arrotonda, ripeti, aspetta, uno_a_caso } da "fratm"` gives small
helpers; the compiler inlines them. `chiamm { gomorra } da "fratm"` adds SQL in Neapolitan
(GomorraSQL): ``gomorra`ripigliammo nomme mmiez 'a pizze arò id = ${id}` `` is translated at compile
time to `{ sql, params, text, values }`, with every `${…}` as a bound parameter. `apri_db(file)`
runs them on SQLite (Node 22.13+): `db.ripiglia(q)`, `db.ripiglia_uno(q)`, `db.esegui(q)`,
`db.chiude()`. Run `fratm guida` for the keyword list.

## Example

```fratm
chiamm { arrotonda } da "fratm"

na famiglie Conto {
    costruttore(nome) {
        stu cos.nome = nome
        stu cos.voci = []
    }

    aggiungi(cosa, prezzo) {
        stu cos.voci.push({ cosa, prezzo })
        piglie stu cos
    }

    totale() {
        piglie stu cos.voci.reduce((somma, v) => somma + v.prezzo, 0)
    }
}

chist è conto = nu bell Conto("Tavolo 4")
conto.aggiungi("Margherita", 6.5).aggiungi("Birra", 4)

pe ogni (chist è { cosa, prezzo } dint conto.voci) {
    stamm a dì(`${cosa}: ${prezzo} euro`)
}

chist è totale = arrotonda(conto.totale(), 2)
si (totale > 10) {
    stamm a dì(`Totale ${totale} euro`)
} sinnò {
    stamm a dì("Poco poco")
}
```
