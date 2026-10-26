# create-fratm

Creates a [FratmScript](https://github.com/kkratterf/fratmscript) project without installing the `fratm` CLI:

```bash
npm create fratm@latest pizzeria
cd pizzeria
npm install
npm run dev
```

The default project is a React app built with Vite, written in `.fratm` files with JSX.

| Option | What you get |
| --- | --- |
| `--completa` | React plus a Hono API and a SQLite database (Node 22.13+) |
| `--senza-react` | the same Vite setup with plain DOM code |
| `--semplice` | only a `main.fratm` |
| `--tailwind` | adds Tailwind CSS to the React app |

The templates are the ones of `fratm nuovo`: they come from the compiler itself (the `fratmscript` package), so the two never drift apart.
