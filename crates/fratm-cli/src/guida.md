# FratmScript reference for AI agents

FratmScript is JavaScript with Neapolitan keywords. Source files end in `.fratm`; the compiler
(written in Rust) turns them into plain modern JavaScript (ES modules) that runs on Node.js or in
the browser. Only the keywords change: operators, literals, numbers, strings, regexes, method
calls and every JavaScript built-in (`Math`, `JSON`, `Array`, `Promise`, `Map`, `Set`, `fetch`,
`process`, `console`, `Object`, `Date`...) are written exactly as in JavaScript.

Playground: https://kkratterf.github.io/fratmscript/ · Repo: https://github.com/kkratterf/fratmscript

## Rules that trip up agents

1. **JavaScript keywords are not keywords here.** Never write `const`, `let`, `var`, `function`,
   `return`, `if`, `else`, `for`, `while`, `new`, `class`, `this`, `true`, `false`, `null`,
   `try`, `catch`, `throw`, `async`, `await`, `import`, `export`. Use the dialect words in the
   table below. (`typeof` and `instanceof` are accepted as aliases.)
2. **One statement per line**, or separate statements with `;`. Semicolons are optional, and a
   line that starts with `[` or `(` always starts a new statement (no JavaScript ASI trap; do
   not put a defensive `;` at the start of a line).
3. **A line ends the statement unless the expression clearly goes on**: a line may end with a
   binary operator, `=`, `?`, `:` or `=>`, or the next line may start with `.method()`, `?.`,
   `?`, `:`, `&&`, `||`, `??` or a comparison; inside `( )` and `[ ]` line breaks never matter.
   `e`/`o` (the words) at the end of a line do not continue it: use `&&`/`||` there, or put
   `e`/`o` at the start of an expression inside parentheses.
4. **Braces are required** after `si`, `sinnò`, `pe`, `mentre che`: `si (x) { ... }`.
5. `chist è` (const) cannot be reassigned; declaring the same name twice in a scope is a compile
   error. Use `tien` (let) for values that change.
6. Labels and tagged templates work as in JavaScript: `fore: pe (...) { rompe fore }`,
   `` String.raw`a\nb` ``.
7. Compiler and runtime error messages are in Neapolitan, with line and column. See "Reading
   errors" below; most errors also carry a hint that starts with 💡.

## Keywords

| FratmScript | JavaScript |
| --- | --- |
| `chist è` | `const` |
| `tien` | `let` |
| `facc` | `function` |
| `piglie` | `return` |
| `si` / `sinnò` | `if` / `else` |
| `vedimm` / `caso` / `predefinit` | `switch` / `case` / `default` |
| `mentre che` | `while` |
| `primm fa { } mentre che (c)` | `do { } while (c)` |
| `pe` (also `pe ogni`) | `for` |
| `pe aspett` | `for await` |
| `dint` / `'ncopp` | `of` / `in` (only inside a `pe` header) |
| `rompe` / `salta` | `break` / `continue` |
| `overo` / `sfòls` | `true` / `false` |
| `nisciun` / `boh` | `null` / `undefined` |
| `stamm a dì(x)` | `console.log(x)` |
| `avvis a dì(x)` / `scrive a dì(x)` | `console.warn(x)` / `console.error(x)` |
| `mo vir` / `aspett` | `async` / `await` |
| `facc*` / `caccia` / `caccia*` | `function*` / `yield` / `yield*` |
| `pruvamm` / `e si schiatta (e)` / `aropp` | `try` / `catch (e)` / `finally` |
| `iett` | `throw` |
| `nu bell` | `new` |
| `na famiglie` / `figlio 'e` | `class` / `extends` |
| `costruttore` | `constructor` |
| `'o pate` | `super` |
| `stu cos` | `this` |
| `fisso` | `static` |
| `damme` / `miette` | `get` / `set` (class members) |
| `chiamm { a } da "m"` | `import { a } from "m"` |
| `chiamm X da "m"` / `chiamm * comme ns da "m"` | `import X from "m"` / `import * as ns from "m"` |
| `comme` (also `as`) | `as` (in imports and exports) |
| `mann for` / `mann for predefinit` | `export` / `export default` |
| `e` (also `pure`, `&&`) | `&&` |
| `o` (also `\|\|`) | `\|\|` |
| `no` (also `manco`, `!`) | `!` |
| `chè è x` | `typeof x` |
| `x è uno T` | `x instanceof T` |
| `"k" dint'a o` | `"k" in o` |
| `leva` | `delete` |
| `fermete` | `debugger` |

No accents needed: `sinno`, `sfols`, `stamm a di`, `chist e` work too. Short words such as `e`,
`o`, `no`, `da`, `caso` are only keywords where their construct starts, so they also work as
names (`e si schiatta (e)`). Names reserved in JavaScript are renamed in the output (`tien for`
becomes `let $for`).

## Variables and values

```fratm
chist è nome = "Gennaro"
tien contatore = 0
tien x = 1, y = 2
contatore += 1
contatore++
chist è attivo = overo
chist è vuoto = nisciun
chist è boh_che_è = boh
chist è saluto = `Uè ${nome}, tieni ${contatore} pizze`
chist è lista = [1, 2, 3]
chist è pizza = { nome: "Margherita", prezzo: 6.5 }
chist è tipo = chè è nome
stamm a dì(saluto, tipo, lista è uno Array)
```

## Functions

```fratm
facc somma(a, b) {
    piglie a + b
}
chist è doppio = (x) => x * 2
chist è saluta = (nome = "Gennà", ...altri) => `Uè ${nome}`
chist è quadrati = [1, 2, 3].map((n) => {
    piglie n * n
})
stamm a dì(somma(2, 3), doppio(4), saluta(), quadrati)

// function expressions (anonymous or named, also mo vir facc and facc*)
chist è triplo = facc (x) { piglie x * 3 }
chist è fatt = facc f(n) { piglie n <= 1 ? 1 : n * f(n - 1) }
stamm a dì(triplo(2), fatt(5))
```

## Control flow

```fratm
chist è voto = 7
si (voto >= 6 e voto < 9) {
    stamm a dì("promosso")
} sinnò si (voto >= 9) {
    stamm a dì("bravo assai")
} sinnò {
    stamm a dì("bocciato")
}

chist è esito = voto > 5 ? "ok" : "no"

vedimm (voto) {
    caso 10:
        stamm a dì("perfetto")
        rompe
    predefinit:
        stamm a dì("normale")
}

pe (tien i = 0; i < 3; i++) {
    si (i == 1) {
        salta
    }
    stamm a dì(i)
}

fore: pe (tien i = 0; i < 3; i++) {
    pe (tien j = 0; j < 3; j++) {
        si (j == 2) {
            salta fore
        }
        si (i == 2) {
            rompe fore
        }
    }
}

pe ogni (chist è frutto dint ["mela", "pera"]) {
    stamm a dì(frutto)
}

pe ogni (chist è chiave 'ncopp { a: 1, b: 2 }) {
    stamm a dì(chiave)
}

tien n = 0
mentre che (n < 3) {
    n++
}
primm fa {
    n--
} mentre che (n > 0)
```

## Objects, destructuring, spread

```fratm
chist è nome = "Margherita"
chist è campo = "prezzo"
chist è pizza = {
    nome,
    [campo]: 6.5,
    ingredienti: ["pummarola", "muzzarella"],
    descrivi() {
        piglie `${stu cos.nome}: ${stu cos.ingredienti.join(", ")}`
    },
    damme quanti() { piglie stu cos.ingredienti.length },  // getter (miette = setter)
}
chist è { prezzo, forno = "a legna", ...altro } = pizza
chist è [primo, ...ato] = pizza.ingredienti
tien a = 1
tien b = 2
[a, b] = [b, a]                     // swap; objects: ({ x, y } = punto)
chist è copia = { ...pizza, prezzo: 7 }
chist è tutti = [...ato, "basilico"]
stamm a dì(pizza.descrivi(), prezzo, forno, primo, copia.prezzo, tutti)
stamm a dì(pizza.cliente?.nome ?? "anonimo")
leva copia.prezzo
```

## Classes

```fratm
na famiglie Persona {
    costruttore(nome) {
        stu cos.nome = nome
    }

    saluta() {
        piglie "Uè, so' " + stu cos.nome
    }

    fisso specie() {
        piglie "Essere umano"
    }
}

na famiglie Pizzaiolo figlio 'e Persona {
    #forno = "a legna"
    pizze = 0
    fisso aperte = 0

    costruttore(nome) {
        'o pate(nome)
    }

    damme forno() {
        piglie stu cos.#forno
    }

    miette forno(tipo) {
        stu cos.#forno = tipo
    }
}

chist è ciro = nu bell Pizzaiolo("Ciro")
ciro.forno = "elettrico"
stamm a dì(ciro.saluta(), ciro.forno, Persona.specie(), ciro è uno Persona)

// static block, and a class used as a value
na famiglie Menu {
    fisso pizze = []
    fisso {
        Menu.pizze.push("Margherita")
    }
}
chist è Anonima = na famiglie figlio 'e Persona { }
stamm a dì(Menu.pizze, nu bell Anonima("Totò").saluta())
```

## Errors

```fratm
facc dividi(a, b) {
    si (b == 0) {
        iett nu bell Error("Nun se divide pe zero!")
    }
    piglie a / b
}

pruvamm {
    dividi(1, 0)
} e si schiatta (errore) {
    scrive a dì(errore.message)
} aropp {
    stamm a dì("fatto")
}
```

## Async and generators

```fratm
chist è aspettaUnPoco = (ms) => nu bell Promise((risolvi) => setTimeout(risolvi, ms))

mo vir facc carica() {
    aspett aspettaUnPoco(10)
    piglie "dati"
}

facc* conta(n) {
    pe (tien i = 1; i <= n; i++) {
        caccia i
    }
}

mo vir facc main() {
    chist è dati = aspett carica()
    chist è tutti = aspett Promise.all([carica(), carica()])
    stamm a dì(dati, tutti, [...conta(3)])
}

main()
```

Top-level `aspett` works too, because the output is an ES module.

## Modules

```fratm
chiamm { readFileSync } da "node:fs"
chiamm * comme path da "node:path"
chiamm os da "node:os"
chiamm { a_caso comme dado } da "fratm"

mann for facc leggi(nome) {
    piglie readFileSync(path.join(".", nome), "utf8")
}

chist è VERSIONE = "1.0"
mann for { VERSIONE, leggi comme read }
mann for predefinit leggi

// dynamic import: chiamm("…") gives a Promise
chist è url = aspett chiamm("node:url")
stamm a dì(os.platform(), dado(1, 6), chè è url.pathToFileURL)
```

Also: `chiamm "./stile.css"` (side effects only), `mann for { a } da "./m.js"` and
`mann for * da "./m.js"` (re-exports), `chiamm { predefinit comme App } da "./app.js"`
(`predefinit` means `default` on the module side). `fratm lancia` compiles one file: to import
another `.fratm` file, build each file with `fratm build` and import the `.js`, or use the Vite
plugin.

## JSX: components and web pages

`.fratm` files can contain JSX, so React (or Preact) components are written in FratmScript.
Tags are HTML as usual (`div`, `button`, `input`); a capitalised or dotted tag is a component.
Inside `{ }` goes any FratmScript expression.

```fratm
chiamm { useState } da "react"
chiamm "./stile.css"

facc Pizza({ nomme, prezzo }) {
    piglie <li class="pizza">{nomme}: {prezzo} €</li>
}

mann for predefinit facc Menu({ pizze }) {
    tien [scelte, setScelte] = useState(0)
    piglie (
        <section>
            <h1>Pizze scelte: {scelte}</h1>
            <ul>
                {pizze.map((p) => <Pizza key={p.nomme} {...p} />)}
            </ul>
            {scelte > 0 ? <p>Bravo!</p> : nisciun}
            <button onClick={() => setScelte(scelte + 1)}>Scegli</button>
        </section>
    )
}
```

- Supported: elements, self-closing tags, fragments `<>…</>`, `{expr}` attributes and children,
  `{...props}`, boolean attributes, `{/* comments */}`, HTML entities (`&amp;`, `&nbsp;`).
- `class` and `for` work on HTML tags (they become `className` / `htmlFor`).
- The output uses the automatic JSX runtime: `import { jsx as _jsx, ... } from
  "react/jsx-runtime"` is added at the top. Write `// @jsxImportSource preact` at the top of the
  file to use Preact; the npm package's `compile` takes `jsxImportSource` too.
- Hooks and React APIs keep their names: `useState`, `useEffect`, `createRoot`.
- Styles: import plain CSS (`chiamm "./stile.css"`) or CSS modules
  (`chiamm stile da "./App.module.css"`, then `className={stile.titolo}`) through Vite; inline
  styles are objects: `style={{ color: "red" }}`.
- A `<` starts JSX only where a value can start (after `=`, `(`, `,`, `piglie`, `=>`, ...) and
  when a tag name or `>` follows right away, so `a < b` and `i <n` stay comparisons.
- To try a component, open the playground: code that imports `"react"` and exports a default
  component is drawn in a live preview, with its `console.log` output in the console. Run a
  whole app with `fratm nuovo`, or a full-stack one with `fratm nuovo --completa`: React in `src/`,
  a Hono API in `server/api.fratm` with SQLite (`apri_db` + `gomorra`), one `npm run dev`, and
  `npm start` (`node --import fratmscript/node server/avvia.fratm`) in production.

## The `fratm` library

Import helpers from `"fratm"`: the compiler inlines their code, no package is needed.

```fratm
chiamm { a_caso, uno_a_caso, mischia, quanti, arrotonda, ripeti, aspetta } da "fratm"

stamm a dì(a_caso(1, 6))
stamm a dì(uno_a_caso(["Totò", "Peppino"]))
stamm a dì(mischia([1, 2, 3]))
stamm a dì(quanti("pizza"), quanti({ a: 1 }))
stamm a dì(arrotonda(3.14159, 2))
ripeti(3, (i) => stamm a dì("giro", i))
aspett aspetta(10)
```

| Helper | Does |
| --- | --- |
| `a_caso(min, max)` | random integer, both ends included |
| `uno_a_caso(lista)` | random item of a list |
| `mischia(lista)` | shuffled copy |
| `quanti(cosa)` | length of a list or string, size of a Set/Map, number of object keys |
| `arrotonda(n, cifre = 0)` | round to `cifre` decimals |
| `ripeti(volte, fa)` | call `fa(0)` ... `fa(volte - 1)` |
| `aspetta(ms)` | Promise resolved after `ms` milliseconds |
| ``gomorra`…` `` | SQL query in Neapolitan (next section) |
| `apri_db(file = ":memory:")` | SQLite database on `node:sqlite` (Node 22.13+; next section) |

## GomorraSQL: SQL queries

`` gomorra`…` `` (imported from `"fratm"`) is SQL written in Neapolitan, translated at compile time:
a wrong query is a compile error with its line. Each `${…}` is a bound parameter, never pasted into
the SQL. The value is `{ sql, params, text, values }`: `sql`/`params` with `?` (SQLite, MySQL),
`text`/`values` with `$1` (PostgreSQL `pg`).

```fratm
chiamm { gomorra } da "fratm"

chist è minimo = 6
chist è q = gomorra`
    ripigliammo nomme, prezzo mmiez 'a pizze
    arò prezzo >= ${minimo} e nomme nun è nisciun
    mettimmo in fila pe prezzo a scennere
    nun cchiù 'e 10 zumpanno 0
`
stamm a dì(q.sql, q.params)
stamm a dì(gomorra`nzipp 'ngoppa pizze nomme, prezzo chist ${"margherita"}, ${5.5}`.sql)
stamm a dì(gomorra`rifacimm pizze accunza prezzo accussì ${7} arò nomme assumiglia a ${"%fritta%"}`.sql)
stamm a dì(gomorra`facimm na' strage mmiez 'a pizze arò id dint'a (${1}, ${2})`.sql)
stamm a dì(gomorra`facimm 'a tavula pizze (id INTEGER PRIMARY KEY, nomme TEXT, prezzo REAL)`.sql)
```

| GomorraSQL | SQL |
| --- | --- |
| `ripigliammo` · `tutto chillo ch'era 'o nuostro` · `mmiez 'a` · `pesc e pesc` · `arò` | `SELECT` · `*` · `FROM` · `JOIN` · `WHERE` |
| `e` · `o` · `nun` · `è` · `nun è` · `nisciun` · `overo` · `sfòls` | `AND` · `OR` · `NOT` · `IS` · `IS NOT` · `NULL` · `TRUE` · `FALSE` |
| `assumiglia a` · `dint'a` · `comme` | `LIKE` · `IN` · `AS` |
| `mettimmo in fila pe` · `a salì` · `a scennere` · `facimm 'e mucchie pe` | `ORDER BY` · `ASC` · `DESC` · `GROUP BY` |
| `nun cchiù 'e` · `zumpanno` | `LIMIT` · `OFFSET` |
| `nzipp 'ngoppa t cols chist vals` | `INSERT INTO t (cols) VALUES (vals)` |
| `rifacimm t accunza c accussì v` | `UPDATE t SET c = v` |
| `facimm na' strage mmiez 'a t` | `DELETE FROM t` |
| `facimm 'a tavula t (…)` · `abbrucimm 'a tavula t` | `CREATE TABLE IF NOT EXISTS` · `DROP TABLE IF EXISTS` |
| `ua uagliò` · `iamme bello ia'` · `sfaccimm` | `BEGIN` · `COMMIT` · `ROLLBACK` |

- Strings inside a query use `"…"` (an apostrophe starts Neapolitan words like `'a`). Prefer `${…}` for values.
- Other words pass through as SQL: column names, `COUNT(*)`, `PRIMARY KEY`, `RETURNING id`.
- Table and column names cannot be `${…}`; one statement per query, no `;`.
- `gomorra` only works as a tag right before a template.

`apri_db` runs the queries on SQLite (Node's built-in `node:sqlite`, Node 22.13 or newer, nothing to
install). It accepts a `gomorra` query or plain SQL with `?` followed by the values.

```fratm
chiamm { apri_db, gomorra } da "fratm"

chist è db = apri_db()   // in memory; apri_db("pizzeria.db") for a file
db.esegui(gomorra`facimm 'a tavula pizze (id INTEGER PRIMARY KEY, nomme TEXT, prezzo REAL)`)
chist è r = db.esegui(gomorra`nzipp 'ngoppa pizze nomme, prezzo chist ${"Margherita"}, ${5}`)
stamm a dì(r.cambiate, r.ultimo_id)
stamm a dì(db.ripiglia(gomorra`ripigliammo nomme, prezzo mmiez 'a pizze arò prezzo < ${10}`))
stamm a dì(db.ripiglia_uno(gomorra`ripigliammo nomme mmiez 'a pizze arò id = ${1}`))
stamm a dì(db.ripiglia("SELECT COUNT(*) AS n FROM pizze WHERE prezzo > ?", 3))
db.chiude()
```

- `ripiglia` → list of row objects, `ripiglia_uno` → first row or `nisciun`, `esegui` →
  `{ cambiate, ultimo_id }`, `chiude()` closes.
- The methods are synchronous; `aspett db.ripiglia(…)` also works (and stays right if the driver
  becomes async). In the browser or the playground `apri_db` throws: it needs Node.

## Reading errors

Errors look like `Riga 3, colonna 7: <message>` (line 3, column 7). Common messages:

| Neapolitan message (start) | Meaning |
| --- | --- |
| `Ccà ce vo' 'na espressione, ma aggio truvato '*'` | expected an expression; the token found is quoted |
| `Ccà ce sta 'x' ma l'istruzione era già fernuta` | unexpected token after a complete statement (often: a JavaScript keyword) |
| `Ccà ce vo' 'na virgola ',' (o ')' pe chiudere)` | missing comma between arguments, array items or object properties |
| `Manca 'a parentesi chiusa` / `parentesi graffa ... nun l'hai chiusa` | missing `)` / `}` at the end of the file |
| `Ccà ce vo' 'x', ma aggio truvato 'y'` | expected `x` (for example `{`: braces are required) |
| `... ma aggio truvato 'a fine d'a riga` / `'a fine d'o file` | the line or the file ended too early |
| `Stu carattere '@' nun 'o canosco` | a character FratmScript does not use |
| `Ccà ce vo' nu nome` | expected a name (often: a keyword used as a variable name) |
| `'x' già esiste: l'hai dichiarato a riga N` | name declared twice |
| `'x' nun esiste` | runtime: name not defined |
| `Nun se pò cagnà na costante` | runtime: assignment to a `chist è` constant |
| `'x' nun è na funzione` | runtime: not a function |
| `In FratmScript 'const' se scrive 'chist è'` | hint: replace the JavaScript keyword |

Runtime errors keep the original JavaScript message in brackets.

## Tools

```text
fratm nuovo app                         new React + Vite app in ./app (--completa, --senza-react, --semplice)
fratm lancia main.fratm                 compile and run with Node.js (alias: run)
fratm build main.fratm [-o out.js]      write JavaScript (--stdout prints it)
fratm check a.fratm b.fratm             report errors, write nothing
fratm fmt main.fratm                    format in place (--check to only report)
fratm guida                             print this reference (alias: reference)
fratm mcp                               MCP server on stdio (see below)
```

For programs and agents: `check`, `build` and `lancia` take `--json` and print one JSON object
with `ok` and errors as `{ message, line, column, suggestion }`: `build` gives `errors` and
`code`; `lancia` gives `errors`, `stdout`, `stderr`, `exitCode` and `runtimeError`; `check`
gives `files`, one `{ file, ok, errors }` per file. Use `-` as the file name to read the
program from stdin, and `lancia --timeout <seconds>` to stop programs that do not end.

```text
echo 'stamm a dì(1 + 1)' | fratm lancia --json -
{"errors":[],"exitCode":0,"ok":true,"runtimeError":null,"stderr":"","stdout":"2\n"}
```

`fratm mcp` is a Model Context Protocol server with the tools `fratm_reference`, `fratm_check`,
`fratm_compile`, `fratm_run` (10 s timeout by default) and `fratm_format`; each takes `source`
or `path`. Add it to Claude Code with `claude mcp add fratm -- fratm mcp`, or to any MCP client
as the command `fratm` with the argument `mcp`.

Install the CLI: `curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh`
(macOS/Linux) or `cargo install --git https://github.com/kkratterf/fratmscript fratm-cli`.
In JavaScript: the `fratmscript` npm package (`compile(source)`), its Vite plugin
(`fratmscript/vite`) and `node --import fratmscript/node main.fratm` to run `.fratm` in Node.

## A complete program

```fratm
chiamm { arrotonda } da "fratm"

na famiglie Pizzeria {
    costruttore(nome) {
        stu cos.nome = nome
        stu cos.ordini = []
    }

    ordina(pizza, prezzo) {
        stu cos.ordini.push({ pizza, prezzo })
        piglie stu cos
    }

    totale() {
        piglie stu cos.ordini.reduce((somma, o) => somma + o.prezzo, 0)
    }
}

chist è daCiro = nu bell Pizzeria("Da Ciro")
daCiro.ordina("Margherita", 6.5).ordina("Marinara", 5)

pe ogni (chist è { pizza, prezzo } dint daCiro.ordini) {
    stamm a dì(`${pizza}: ${prezzo} euro`)
}

chist è conIva = arrotonda(daCiro.totale() * 1.1, 2)
si (conIva > 10) {
    stamm a dì(`Totale ${conIva} euro: grazie assai!`)
} sinnò {
    stamm a dì("Tutto qua?")
}
```
