# FratmScript Playground

Interactive web editor to try FratmScript directly in the browser.

Built with **Vite + React + TypeScript + Tailwind CSS v3 + shadcn/ui**.

## Development

### Prerequisites

- Node.js 20+
- pnpm
- Rust + wasm-pack (for WASM compilation)

### Setup

```bash
# Install dependencies
pnpm install

# Build WASM module (from project root)
cd crates/fratm-wasm
wasm-pack build --target web --out-dir ../../packages/playground/public/pkg

# Start development server
cd packages/playground
pnpm dev
```

The playground will be available at `http://localhost:5173`.

### Build for Production

```bash
pnpm build
```

Output will be in the `dist/` folder.

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

## Tech Stack

- **Vite** - Fast build tool
- **React 18** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS v3** - Utility-first CSS
- **shadcn/ui** - Component library
- **Lucide React** - Icons
