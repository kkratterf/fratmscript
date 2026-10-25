# fratmscript

The [FratmScript](https://github.com/kkratterf/fratmscript) compiler as an npm package: the real Rust compiler built to WebAssembly, a small JS API and a Vite plugin.

```bash
npm i fratmscript
```

## Compile from Node

```js
import { compile, format } from 'fratmscript';

const { code, map, errors } = compile('stamm a dì("Uè!")', { filename: 'main.fratm' });
// code   → 'console.log("Uè!");\n'
// map    → source map v3 object (pass { sourceMap: false } to skip it)
// errors → [] or [{ message, line, column, suggestion }], messages in Neapolitan

format('si (overo) {\nstamm a dì(1)\n}').code; // same output as `fratm fmt`
```

`compile` never throws on invalid source: check `errors`.

Errors thrown by the compiled code can be shown in Neapolitan too:

```js
import { runtimeError } from 'fratmscript';

try {
  run(code);
} catch (e) {
  console.error(runtimeError(e));
  // 'nonEsiste' nun esiste: nun l'hê maje dichiarato cu 'chist è' o 'tien' (ReferenceError: nonEsiste is not defined)
}
```

## Vite plugin

```js
// vite.config.js
import fratmscript from 'fratmscript/vite';

export default {
  plugins: [fratmscript()],
};
```

Then import `.fratm` files from your code, and from each other with `chiamm { … } da "./file.fratm"`:

```js
import { saluta } from './saluto.fratm';
```

Compile errors show up in the Vite overlay with their line and column, and source maps point at the `.fratm` files. The plugin has no dependency on Vite, so it also works as a Rollup plugin.

For React Fast Refresh on `.fratm` components, add `@vitejs/plugin-react` after it and let it see the compiled files:

```js
import react from '@vitejs/plugin-react';

export default {
  plugins: [fratmscript(), react({ include: /\.(fratm|[jt]sx?)$/ })],
};
```

### An API on the dev server

`api` mounts a server written in FratmScript on the Vite dev server, so one `npm run dev` serves both the app and its API. The module's default export must have a `fetch(request)` method that returns a `Response`, as a [Hono](https://hono.dev) app does. It answers the requests under `/api`, and it is reloaded when you edit it.

```js
export default {
  plugins: [fratmscript({ api: 'server/api.fratm' })],
  // or fratmscript({ api: { entry: 'server/api.fratm', prefix: '/dati' } })
};
```

```fratm
// server/api.fratm
chiamm { Hono } da "hono"
chist è api = nu bell Hono().basePath("/api")
api.get("/pizze", (c) => c.json(["Margherita", "Marinara"]))
mann for predefinit api
```

In production run the same module with `node --import fratmscript/node` and a Node adapter such as `@hono/node-server`. `fratm nuovo --completa` sets all of this up.

## Run .fratm files in Node

```bash
node --import fratmscript/node server.fratm
```

`fratmscript/node` registers a module hook (Node 20.6 or later) that compiles every `.fratm` file when it is loaded, so `.fratm` modules can import each other, npm packages and `node:` modules. Stack traces point at the `.fratm` lines, and compile errors are printed in Neapolitan with file, line and column.

## In a web page, with a script tag

No build step: load `browser.js` and write FratmScript in `<script type="text/fratmscript">` tags, inline or with `src`. They run in document order once the page has loaded; compile errors and uncaught runtime errors are printed in Neapolitan in the console, and dev tools show the `.fratm` source.

```html
<meta charset="utf-8">
<script type="module" src="https://cdn.jsdelivr.net/npm/fratmscript/browser.js"></script>

<script type="text/fratmscript">
  chist è nome = "Gennaro"
  stamm a dì(`Uè ${nome}!`)
</script>
<script type="text/fratmscript" src="pizzeria.fratm"></script>
```

`import { ready } from 'fratmscript/browser'` gives a promise that resolves when every script has run. Code that uses `chiamm`/`mann for` runs as a module.

## In the browser, from JavaScript

```js
import { init, compile } from 'fratmscript/web';

await init(); // fetches the .wasm file next to the module
compile('tien x = 1').code;
```
