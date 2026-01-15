# FratmScript Playground

Editor web interattivo per provare FratmScript direttamente nel browser.

## Come Usare

### Senza Build (Demo Mode)

Apri semplicemente `index.html` nel browser. Il playground funzionera in modalita demo con un transpiler semplificato basato su regex.

### Con WASM (Full Mode)

Per usare il compilatore completo:

```bash
# 1. Installa wasm-pack (se non gia installato)
cargo install wasm-pack

# 2. Build del modulo WASM
cd crates/fratm-wasm
wasm-pack build --target web --out-dir ../../packages/playground/pkg

# 3. Avvia un server locale (necessario per i moduli ES)
cd ../../packages/playground
python3 -m http.server 8080
# oppure: npx serve .

# 4. Apri http://localhost:8080 nel browser
```

## Funzionalita

- **Editor con sintassi FratmScript**: scrivi il tuo codice napoletano
- **Compilazione istantanea**: premi `Ctrl+Enter` o clicca "Esegui"
- **Output JavaScript**: vedi il codice JS generato
- **Console**: vedi l'output del programma
- **Esempi precaricati**: seleziona dal menu dropdown
- **Condivisione**: genera un link condivisibile con il codice

## Keyboard Shortcuts

| Shortcut | Azione |
|----------|--------|
| `Ctrl+Enter` | Esegui codice |
| `Tab` | Indenta |

## Esempi Disponibili

1. **Hello World** - variabili e output
2. **Fibonacci** - funzioni ricorsive
3. **Classe Pizzaiolo** - classi e metodi
4. **Async/Await** - funzioni asincrone
5. **Operatori Logici** - e, o, no, pure, manco
6. **Array e Oggetti** - strutture dati
