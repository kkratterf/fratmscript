# FratmScript VSCode Extension

Supporto completo per **FratmScript** - JavaScript scritto in Napoletano.

## Funzionalità

### Syntax Highlighting
Evidenziazione completa per tutte le keywords napoletane:
- `chist ... è` → `const`
- `tien` → `let`
- `facc` → `function`
- `si / sinnò` → `if / else`
- `mentre che` → `while`
- `stamm a dì` → `console.log`
- E molte altre...

### Snippets
30+ snippets per scrivere codice velocemente:
- `chist` → Dichiarazione costante
- `facc` → Funzione
- `si` → Blocco if
- `nafamiglie` → Classe
- `pruvamm` → Try-catch

### Language Server
- **Autocompletamento** intelligente per tutte le keywords
- **Hover documentation** con traduzioni JS
- **Diagnostics** in tempo reale:
  - Avvisi se usi keywords JavaScript invece di napoletane
  - Errori per parentesi non bilanciate
- **Semantic highlighting** per una migliore leggibilità

## Installazione

### Da VSCode Marketplace
Cerca "FratmScript" nel marketplace VSCode.

### Sviluppo locale
```bash
cd packages/vscode-extension
npm install
npm run compile
```

Poi premi `F5` in VSCode per lanciare l'extension host.

## Configurazione

```json
{
  "fratmscript.enableDiagnostics": true,
  "fratmscript.showTranslationOnHover": true
}
```

## Keywords Reference

| Napoletano | JavaScript | Descrizione |
|------------|------------|-------------|
| `chist ... è` | `const` | Costante |
| `tien` | `let` | Variabile |
| `facc` | `function` | Funzione |
| `piglie` | `return` | Return |
| `si` | `if` | If |
| `sinnò` | `else` | Else |
| `mentre che` | `while` | While |
| `pe ogni` | `for` | For |
| `overo` | `true` | True |
| `sfòls` | `false` | False |
| `nisciun` | `null` | Null |
| `boh` | `undefined` | Undefined |
| `stamm a dì` | `console.log` | Log |
| `na famiglie` | `class` | Classe |
| `nu bell` | `new` | New |
| `chiamm ... da` | `import ... from` | Import |
| `mann for` | `export` | Export |
| `mo vir facc` | `async function` | Async |
| `aspett` | `await` | Await |
| `pruvamm` | `try` | Try |
| `e si schiatta` | `catch` | Catch |
| `iett` | `throw` | Throw |

## Esempio

```fratm
// Fibonacci in napoletano
facc fibonacci(n) {
  si (n <= 1) {
    piglie n
  }
  piglie fibonacci(n - 1) + fibonacci(n - 2)
}

chist risultato è fibonacci(10)
stamm a dì(risultato)
```

## License

MIT
