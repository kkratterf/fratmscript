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

## In the browser

```js
import { init, compile } from 'fratmscript/web';

await init(); // fetches the .wasm file next to the module
compile('tien x = 1').code;
```
