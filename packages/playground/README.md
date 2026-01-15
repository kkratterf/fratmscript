# FratmScript Playground

Interactive web editor to try FratmScript directly in the browser.

## How to Use

### Without Build (Demo Mode)

Simply open `index.html` in the browser. The playground will work in demo mode with a simplified regex-based transpiler.

### With WASM (Full Mode)

To use the full compiler:

```bash
# 1. Install wasm-pack (if not already installed)
cargo install wasm-pack

# 2. Build the WASM module
cd crates/fratm-wasm
wasm-pack build --target web --out-dir ../../packages/playground/pkg

# 3. Start a local server (required for ES modules)
cd ../../packages/playground
python3 -m http.server 8080
# or: npx serve .

# 4. Open http://localhost:8080 in the browser
```

## Features

- **Editor with FratmScript syntax**: write your Neapolitan code
- **Instant compilation**: press `Ctrl+Enter` or click "Run"
- **JavaScript output**: see the generated JS code
- **Console**: see the program output
- **Preloaded examples**: select from the dropdown menu
- **Sharing**: generate a shareable link with the code

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Enter` | Run code |
| `Tab` | Indent |

## Available Examples

1. **Hello World** - variables and output
2. **Fibonacci** - recursive functions
3. **Pizzaiolo Class** - classes and methods
4. **Async/Await** - asynchronous functions
5. **Logical Operators** - e, o, no, pure, manco
6. **Arrays and Objects** - data structures
