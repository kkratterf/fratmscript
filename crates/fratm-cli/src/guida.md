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
2. **One statement per line**, or separate statements with `;`. Semicolons are optional.
3. **An expression cannot continue on the next line** after a binary operator, after `?` or `:`,
   or on a line that starts with `.method()`. Keep each expression on one line, or split it with
   intermediate `chist è` variables. Array literals, object literals, call arguments and `{ }`
   blocks may span several lines.
4. **Braces are required** after `si`, `sinnò`, `pe`, `mentre che`: `si (x) { ... }`.
5. `chist è` (const) cannot be reassigned; declaring the same name twice in a scope is a compile
   error. Use `tien` (let) for values that change.
6. Not supported yet: the `in` operator (use `Object.hasOwn(obj, "k")`), unary `+` (use
   `Number(x)`), bitwise operators, `void`, BigInt literals, labels, tagged templates, class
   expressions, getters/setters in object literals, default and namespace imports
   (`import x from`, `import * as`): only named imports `chiamm { a, b } da "m"`.
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
| `mann for` / `mann for predefinit` | `export` / `export default` |
| `e` (also `pure`, `&&`) | `&&` |
| `o` (also `\|\|`) | `\|\|` |
| `no` (also `manco`, `!`) | `!` |
| `chè è x` | `typeof x` |
| `x è uno T` | `x instanceof T` |
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
    }
}
chist è { prezzo, forno = "a legna", ...altro } = pizza
chist è [primo, ...ato] = pizza.ingredienti
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
chiamm { join } da "node:path"

mann for facc leggi(nome) {
    piglie readFileSync(join(".", nome), "utf8")
}

mann for chist è VERSIONE = "1.0"
mann for predefinit leggi
```

Only named imports exist today. `fratm lancia` compiles one file: to import another `.fratm` file, build each file with `fratm build` and import the `.js`, or use the Vite plugin.

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

## Reading errors

Errors look like `Riga 3, colonna 7: <message>` (line 3, column 7). Common messages:

| Neapolitan message (start) | Meaning |
| --- | --- |
| `Ccà ce vo' 'na espressione` | expected an expression (often: expression split over two lines) |
| `Ccà ce sta 'x' ma l'istruzione era già fernuta` | unexpected token after a complete statement (often: a JavaScript keyword) |
| `Manca 'a parentesi chiusa` / `parentesi graffa ... nun l'hai chiusa` | missing `)` / `}` |
| `Ccà ce vo' '{', ma aggio truvato ...` | expected `{` (braces are required) |
| `Ccà ce vo' nu nome` | expected a name (often: a keyword used as a variable name) |
| `'x' già esiste: l'hai dichiarato a riga N` | name declared twice |
| `'x' nun esiste` | runtime: name not defined |
| `Nun se pò cagnà na costante` | runtime: assignment to a `chist è` constant |
| `'x' nun è na funzione` | runtime: not a function |
| `In FratmScript 'const' se scrive 'chist è'` | hint: replace the JavaScript keyword |

Runtime errors keep the original JavaScript message in brackets.

## Tools

```text
fratm lancia main.fratm                 compile and run with Node.js (alias: run)
fratm build main.fratm [-o out.js]      write JavaScript (--stdout prints it)
fratm check a.fratm b.fratm             report errors, write nothing
fratm fmt main.fratm                    format in place (--check to only report)
fratm guida                             print this reference (alias: reference)
```

For programs and agents: `check`, `build` and `lancia` take `--json` and print one JSON object
(`ok`, `errors` with `message`, `line`, `column`, `suggestion`; `build` adds `code`; `lancia`
adds `stdout`, `stderr`, `exitCode` and `runtimeError`). Use `-` as the file name to read the
program from stdin, and `lancia --timeout <seconds>` to stop programs that do not end.

```text
echo 'stamm a dì(1 + 1)' | fratm lancia --json -
{"errors":[],"exitCode":0,"ok":true,"runtimeError":null,"stderr":"","stdout":"2\n"}
```

Install the CLI: `curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh`
(macOS/Linux) or `cargo install --git https://github.com/kkratterf/fratmscript fratm-cli`.
In JavaScript: the `fratmscript` npm package (`compile(source)`) and its Vite plugin.

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
